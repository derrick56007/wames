use std::{
    collections::{HashMap, HashSet},
    io::{self, stdin, stdout, BufRead, Read, Stdin, Write},
    ops::Range,
    os::fd::AsFd,
    thread::{self, sleep}, time::Duration,
};

use components::Position;
use device_query::{DeviceQuery, DeviceState, Keycode};
use entity::new_entity;
use rand::{rngs::ThreadRng, Rng};
use wurdle::{play, wurdle_words};

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

    let mut last: Option<Keycode> = None;
    let device_state = DeviceState::new();

    for (system, components) in &systems {
        system(&component_map[components], &mut state, &mut entities_map);
    }

    'outer: loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let mut pressed_key: Option<Keycode> = None;
        for key in keys.iter() {
            if Some(key) == last.as_ref() {
                // ignore if same key is pressed twice
                continue 'outer;
            }
            pressed_key = Some(*key);
            last = pressed_key;
            match key {
                Keycode::Up => {
                    // new_mino = Some(tetrimino.rotate_right());
                    // break 'outer;
                }
                Keycode::Right => {
                    // new_pos = Some((position.0 + 1, position.1));
                    // break 'outer;
                }
                Keycode::Left => {
                    // if position.0 > 0 {
                    // new_pos = Some((position.0 - 1, position.1));
                    // break 'outer;
                    // }
                }
                Keycode::Down => {


                    let tries = 6;
                    let available_letters: HashSet<char> = HashSet::from_iter("".chars());
                    let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
                        .split("\n")
                        .map(|s| s.to_uppercase())
                        .filter(|word| {
                            if !available_letters.is_empty() {
                                word.chars().all(|c| available_letters.contains(&c))
                            } else {
                                true
                            }
                        })
                        .collect();

                        sleep(Duration::from_millis(100));

                    play(tries, available_letters, words_vec);
                }
                _ => {}
            }
            break;
        }
        if pressed_key.is_none() {
            last = None;
            continue 'outer;
        }

        for (system, components) in &systems {
            system(&component_map[components], &mut state, &mut entities_map);
        }
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
