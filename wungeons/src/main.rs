use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    process,
    thread::sleep,
    time::{Duration, SystemTime},
};

use components::{Item, Position, DIRECTIONS};
use device_query::{DeviceQuery, Keycode};
use entity::new_entity;
use event::Event;

use render::bresenham;
use rooms::{create_floor, create_item};
use wurdle::{play, wurdle_words};

use crate::{
    components::{Component, Rect},
    entity::add_entity,
    inputs::handle_inputs,
    rooms::create_rooms,
    state::State,
};

mod components;
mod entity;
mod event;
mod inputs;
mod render;
mod rooms;
mod state;

use crate::render::render;

fn main() {
    const GRID_SIZE: Rect = Rect {
        width: 70,
        height: 34,
    };

    let mut systems = get_systems();
    let mut state = State::new(
        GRID_SIZE,
        systems
            .iter()
            .map(|(_, components, _)| components.clone())
            .collect::<Vec<Vec<Component>>>(),
    );

    let mut to_remove = vec![];
    loop {
        let start = SystemTime::now();

        for (i, (system, components, single_shot)) in &mut systems.iter().enumerate() {
            system(&mut state, components);
            if *single_shot {
                to_remove.push(i);
            }
        }

        for i in to_remove.iter().rev() {
            systems.remove(*i);
            state.system_components.remove(*i);
        }
        to_remove.clear();

        state.full_loop_duration = Some(SystemTime::now().duration_since(start).unwrap());
    }
}

pub fn start_up(state: &mut State, _components: &[Component]) {
    state.events.push(Event::Refresh);
    state.events.push(Event::GameStart);
}

pub fn game_events(state: &mut State, _components: &[Component]) {
    loop {
        if let Some(event) = state.events.pop() {
            match event {
                Event::Refresh => {
                    state.refresh();
                }
                Event::GameStart => {
                    // remove all entities
                    let entity_ids: Vec<usize> = state.entities_map.keys().cloned().collect();
                    for id in entity_ids {
                        state.remove_entity(id);
                    }

                    state.rooms.clear();
                    state.hallways.clear();
                    let (rooms, room_entities, hallways) = create_rooms(state);
                    state.rooms.extend(rooms);
                    state.hallways.extend(hallways);
                    for room_entity in room_entities {
                        add_entity(room_entity, state);
                    }
                    // create player
                    let entity = new_entity(
                        &mut state.entity_id_counter,
                        vec![
                            Component::Position(Some(state.rooms[0].0.center(&state.rooms[0].1))),
                            Component::Render(Some('@')),
                            Component::ZIndex(Some(5)),
                            Component::Player,
                        ],
                    );

                    add_entity(entity, state);
                }
            }
        } else {
            return;
        }
    }
}

pub fn calculate_fog(state: &mut State, components: &[Component]) {
    let fog_entities = state.get_entities(components).clone();
    let player = state.get_entities(&[Component::Player]).clone();
    let solid_positions = state
        .get_entities(&[Component::Position(None), Component::Solid])
        .clone();
    let solid_positions = solid_positions
        .iter()
        .map(|e| get_component!(state.entities_map[e], Component::Position).unwrap())
        .collect::<Vec<Position>>();

    for f in fog_entities.iter() {
        let fog = get_component!(state.entities_map[f], Component::Fog).unwrap();
        if state.fog_enabled {
            match fog {
                true => {
                    state
                        .entities_map
                        .get_mut(f)
                        .unwrap()
                        .set_component(Component::Render(Some('░')));
                }
                _ => {
                    state
                        .entities_map
                        .get_mut(f)
                        .unwrap()
                        .set_component(Component::Render(Some(' ')));
                }
            }
        } else {
            state
                .entities_map
                .get_mut(f)
                .unwrap()
                .set_component(Component::Render(None));
        }
    }
    if !state.fog_enabled {
        return;
    }
    for p in player {
        let player_pos = get_component!(state.entities_map[&p], Component::Position).unwrap();
        let mut positions = HashSet::new();
        for (rect, pos, _) in &state.rooms {
            for x in pos.x..pos.x + rect.width {
                for p in bresenham(&player_pos, &Position { x, y: pos.y }) {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    positions.insert(p);
                }
                for p in bresenham(
                    &player_pos,
                    &Position {
                        x,
                        y: pos.y + rect.height,
                    },
                ) {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    positions.insert(p);
                }
            }

            for y in pos.y..pos.y + rect.height {
                for p in bresenham(&player_pos, &Position { x: pos.x, y }) {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    positions.insert(p);
                }
                for p in bresenham(
                    &player_pos,
                    &Position {
                        x: pos.x + rect.width,
                        y,
                    },
                ) {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    positions.insert(p);
                }
            }
        }
        for (pos1, pos2) in &state.hallways {
            for p in bresenham(&player_pos, pos1) {
                if solid_positions.contains(&p) {
                    break;
                }
                positions.insert(p);
            }
            for p in bresenham(&player_pos, pos2) {
                if solid_positions.contains(&p) {
                    break;
                }
                positions.insert(p);
            }
        }

        for f in fog_entities.iter() {
            let fog_pos = get_component!(state.entities_map[f], Component::Position).unwrap();
            if positions.contains(&fog_pos) {
                let visited = get_component!(state.entities_map[f], Component::Fog).unwrap();
                match visited {
                    // components::FogState::Dark(true) => {
                    //     state.entities_map.get_mut(f).unwrap().set_component(Component::Render(Some('*')));
                    // },
                    false => {
                        state
                            .entities_map
                            .get_mut(f)
                            .unwrap()
                            .set_component(Component::Fog(Some(true)));
                    }
                    _ => {} // components::FogState::Clear => {

                            // },
                }
                state
                    .entities_map
                    .get_mut(f)
                    .unwrap()
                    .set_component(Component::Render(None));
            } else {
            }
        }
    }
}

fn get_systems() -> Vec<(fn(&mut State, &[Component]), Vec<Component>, bool)> {
    vec![
        (start_up, vec![], true),
        (game_events, vec![], false),
        (
            dummy,
            vec![Component::Position(None), Component::Solid],
            false,
        ),
        (
            calculate_fog,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
                Component::Fog(None),
            ],
            false,
        ),
        (
            render,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
            ],
            false,
        ),
        (handle_inputs, vec![Component::Player], false),
        // (collisions, vec![], false),
    ]
}

fn dummy(state: &mut State, components: &[Component]) {}
