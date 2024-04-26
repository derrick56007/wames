use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use components::Position;
use entity::new_entity;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Component, Rect},
    entity::{add_entity, Entity},
    rooms::create_rooms,
    state::State,
};

mod components;
mod entity;
mod render;
mod rooms;
mod state;

use crate::render::render;


fn main() {
    let mut rng = rand::thread_rng();

    const GRID_SIZE: Rect = Rect {
        width: 100,
        height: 50,
    };

    let mut entity_id_counter = 0;
    let mut entities_map: HashMap<usize, Box<Entity>> = HashMap::new();
    let mut component_map: HashMap<Vec<Component>, HashSet<usize>> = HashMap::new();
    let systems = get_systems();
    let mut state = State::new(GRID_SIZE);

    for room_entity in create_rooms(&mut rng, &GRID_SIZE, &mut entity_id_counter) {
        add_entity(room_entity, &mut entities_map, &mut component_map, &systems);
    }

    // tick
    for (system, components) in systems {
        system(&component_map[&components], &mut state, &mut entities_map);
    }
}

fn get_systems() -> [(
    fn(&HashSet<usize>, &mut State, &mut HashMap<usize, Box<Entity>>),
    Vec<Component>,
); 1] {
    [(
        render,
        vec![Component::Position(None), Component::Render(None)],
    )]
}
