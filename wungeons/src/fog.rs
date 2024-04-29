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
    components::{self, Component, Rect}, entity::{self, add_entity}, event, get_component, inputs::handle_inputs, render, rooms::{self, create_rooms}, state::State
};


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
                    state.set_component(*f, Component::Render(Some('░')));
                }
                _ => {
                    state.set_component(*f, Component::Render(Some(' ')));
                }
            }
        } else {
            state.set_component(*f, Component::Render(None));
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
                        state.set_component(*f, Component::Fog(Some(true)));
                    }
                    _ => {} // components::FogState::Clear => {

                            // },
                }
                state.set_component(*f, Component::Render(None));
            } else {
            }
        }
    }
}
