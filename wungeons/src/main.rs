use std::{
    collections::{HashMap, HashSet},
    io::{self, stdin, stdout, BufRead, Read, Stdin, Write},
    ops::Range,
    os::fd::AsFd,
    process,
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

use components::{Item, Position, DIRECTIONS};
use device_query::{DeviceQuery, DeviceState, Keycode};
use entity::new_entity;
use event::Event;
use rand::{rngs::ThreadRng, Rng};
use rooms::create_item;
use wurdle::{play, wurdle_words};

use crate::{
    components::{Component, Rect},
    entity::{add_entity, Entity},
    rooms::create_rooms,
    state::State,
};

mod components;
mod entity;
mod event;
mod render;
mod rooms;
mod state;

use crate::render::render;

fn main() {
    const GRID_SIZE: Rect = Rect {
        width: 100,
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
    state.events.push(Event::GameStart);
}

pub fn game_events(state: &mut State, _components: &[Component]) {
    loop {
        if let Some(event) = state.events.pop() {
            match event {
                Event::GameStart => {
                    // remove all entities
                    let entity_ids: Vec<usize> = state.entities_map.keys().cloned().collect();
                    for id in entity_ids {
                        state.remove_entity(id);
                    }

                    state.rooms.clear();
                    let (rooms, room_entities) = create_rooms(state);
                    state.rooms.extend(rooms);
                    for room_entity in room_entities {
                        add_entity(room_entity, state);
                    }
                    // create player
                    let entity = new_entity(
                        &mut state.entity_id_counter,
                        vec![
                            Component::Position(Some(state.rooms[0].0.center(&state.rooms[0].1))),
                            Component::Render(Some('@')),
                            Component::ZIndex(Some(2)),
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

pub fn inputs(state: &mut State, components: &[Component]) {
    let entities = state.component_map[components].clone();
    let minion_entities = state.component_map[&vec![Component::Minion(None)]].clone();
    let item_entities = state.component_map.get(&vec![Component::Item(None)]).unwrap_or(&HashSet::new()).clone();
    let door_entities = state.component_map[&vec![Component::Door]].clone();
    let wall_entities = state.component_map[&vec![Component::Wall]].clone();

    let secret_wall_entities = state.component_map[&vec![Component::SecretWall]].clone();

    // dbg!(&state.component_map);

    let entity_ids: Vec<usize> = state.entities_map.keys().cloned().collect();
    let directions: HashMap<Keycode, Position> = HashMap::from_iter(DIRECTIONS);
    let other_positions = entity_ids
        .iter()
        .map(|e| {
            (
                *e,
                get_component!(state.entities_map[e], Component::Position).unwrap(),
            )
        })
        .collect::<Vec<(usize, Position)>>();

    // 'outer: loop {
        let keys: Vec<Keycode> = state.device_state.get_keys();
        let mut pressed_key: Option<Keycode> = None;
        'outer:
        for key in keys.iter() {
            // if Some(key) == state.last_pressed_key.as_ref() {
            //     // ignore if same key is pressed twice
            //     continue 'outer;
            // }
            pressed_key = Some(*key);
            state.last_pressed_key = pressed_key;
            for e in entities.iter() {
                let entity = &state.entities_map[e];
                let position: Position = get_component!(entity, Component::Position).unwrap();

                match key {
                    Keycode::R => {
                        state.events.push(Event::GameStart);
                    }
                    Keycode::Up | Keycode::Right | Keycode::Left | Keycode::Down => {
                        let new_position = &position + &directions[key];
                        // check for collisions
                        let hits = other_positions
                            .iter()
                            .filter_map(|(i, p)| if p == &new_position { Some(*i) } else { None })
                            .collect::<Vec<usize>>();

                        // if hits.is_empty() {
                        //     state
                        //         .entities_map
                        //         .get_mut(&e)
                        //         .unwrap()
                        //         .set_component(Component::Position(Some(new_position.clone())));
                        // } else {
                            // minion_entities.retain(f)
                            // dbg!(&minion_entities, &hits);
                            // process::exit(0);
                            'check_hits: for hit in hits {
                                if minion_entities.contains(&hit) {
                                    let render: char = get_component!(
                                        &state.entities_map[&hit],
                                        Component::Render
                                    )
                                    .unwrap();
                                    let is_boss = get_component!(
                                        &state.entities_map[&hit],
                                        Component::Minion
                                    )
                                    .unwrap();
                                    state.add_letter(render);

                                    let tries = 6;
                                    let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
                                        .split('\n')
                                        .map(|s| s.to_uppercase())
                                        .collect();

                                    sleep(Duration::from_millis(100));

                                    let won =
                                        play(tries, state.available_letters.clone(), words_vec, Some(render), false);
                                    if won {
                                        // dbg!(state.entities_map[&hit]
                                        //     .contains_component(&Component::Drop(None)));
                                        // process::exit(0);
                                        if dbg!(state.entities_map[&hit]
                                            .contains_component(&Component::Drop(None)))
                                        {
                                            let drop = get_component!(
                                                &state.entities_map[&hit],
                                                Component::Drop
                                            )
                                            .unwrap();
                                            add_entity(
                                                create_item(
                                                    &mut state.entity_id_counter,
                                                    &new_position,
                                                    drop,
                                                ),
                                                state,
                                            );
                                        }
                                        if is_boss {
                                            state.events.push(Event::GameStart);
                                        }
                                        state.remove_entity(hit);
                                    } else {
                                        todo!();
                                    }
                                    break 'outer;
                                } else if item_entities.contains(&hit) {
                                    let item =
                                        get_component!(&state.entities_map[&hit], Component::Item)
                                            .unwrap();
                                    state.items.push(item);
                                    state.remove_entity(hit);
                                    break 'outer;

                                } else if door_entities.contains(&hit) {
                                    // let item = get_component!(
                                    //     &state.entities_map[&hit],
                                    //     Component::Item
                                    // )
                                    // .unwrap();
                                    if state.items.contains(&Item::Key) {
                                        state
                                            .items
                                            .remove(state.items.binary_search(&Item::Key).unwrap());

                                        for hit in &door_entities {
                                            state.remove_entity(*hit);
                                        }
                                    }
                                    break 'outer;

                                } 
                                else if wall_entities.contains(&hit) {
                                    break 'outer;

                                } 
                                // else if secret_wall_entities.contains(&hit) {
                                //     state
                                //     .entities_map
                                //     .get_mut(&e)
                                //     .unwrap()
                                //     .set_component(Component::Position(Some(new_position.clone())));
                                // }
                                // break;
                            }

                            state
                                .entities_map
                                .get_mut(e)
                                .unwrap()
                                .set_component(Component::Position(Some(new_position.clone())));
                        // }
                    }
                    // Keycode::Space => {

                    // }
                    _ => {}
                }
            }
            // break 'outer;
        }
        // if pressed_key.is_none() {
        //     state.last_pressed_key = None;
        //     continue 'outer;
        // }
        // process::exit(code)
        sleep(Duration::from_millis(50));
    // }
}

fn get_systems() -> Vec<(fn(&mut State, &[Component]), Vec<Component>, bool)> {
    vec![
        (start_up, vec![], true),
        (game_events, vec![], false),
        (
            render,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
            ],
            false,
        ),
        (inputs, vec![Component::Player], false),
        // (collisions, vec![], false),
    ]
}

// fn collisions(state: &mut State, components: &Vec<Component>) {}
