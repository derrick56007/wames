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
    components::{Component, Rect}, entity::add_entity, event::game_events, fog::calculate_fog, inputs::handle_inputs, rooms::create_rooms, state::State, systems::get_systems
};

mod components;
mod entity;
mod event;
mod inputs;
mod render;
mod rooms;
mod state;
mod fog;
mod systems;

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
