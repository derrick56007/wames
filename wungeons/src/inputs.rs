use std::{collections::{HashMap, HashSet}, process, thread::sleep, time::Duration};

use device_query::{DeviceQuery, Keycode};
use wurdle::{play, wurdle_words};

use crate::{components::{Component, Item, Position, DIRECTIONS}, entity::add_entity, event::Event, get_component, rooms::{create_floor, create_item}, state::State};


pub fn handle_inputs(state: &mut State, components: &[Component]) {
    let entities = state.component_map[components].clone();
    let minion_entities = state.component_map[&vec![Component::Minion(None)]].clone();
    let item_entities = state
        .component_map
        .get(&vec![Component::Item(None)])
        .unwrap_or(&HashSet::new())
        .clone();
    let door_entities = state.component_map[&vec![Component::Door]].clone();
    let wall_entities = state.component_map[&vec![Component::Wall]].clone();

    let secret_wall_entities = state.component_map[&vec![Component::SecretWall(None)]].clone();

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
    'outer: for key in keys.iter() {
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
                    state.events.push(Event::Refresh);
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
                            let render: char =
                                get_component!(&state.entities_map[&hit], Component::Render)
                                    .unwrap();
                            let is_boss =
                                get_component!(&state.entities_map[&hit], Component::Minion)
                                    .unwrap();
                            state.add_letter(render);

                            let tries = 6;
                            let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
                                .split('\n')
                                .map(|s| s.to_uppercase())
                                .collect();

                            sleep(Duration::from_millis(100));

                            let (won, attempts, word) = play(
                                tries,
                                state.available_letters.clone(),
                                words_vec,
                                Some(render),
                                false,
                            );
                            if won {
                                // dbg!(state.entities_map[&hit]
                                //     .contains_component(&Component::Drop(None)));
                                // process::exit(0);
                                if state.entities_map[&hit]
                                    .contains_component(&Component::Drop(None))
                                {
                                    let drop =
                                        get_component!(&state.entities_map[&hit], Component::Drop)
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
                                // for c in word.chars() {
                                //     state.add_letter(c);
                                // }

                                state.gold += 6 - attempts.len();
                                state.remove_entity(hit);
                            } else {
                                println!("You Lost!");
                                process::exit(0);
                                // todo!();
                            }
                            break 'outer;
                        } else if item_entities.contains(&hit) {
                            let item =
                                get_component!(&state.entities_map[&hit], Component::Item).unwrap();
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
                        } else if wall_entities.contains(&hit) {
                            break 'outer;
                        } else if secret_wall_entities.contains(&hit) {
                            let group =
                                get_component!(&state.entities_map[&hit], Component::SecretWall)
                                    .unwrap();
                            for e in secret_wall_entities.iter() {
                                let groupb =
                                    get_component!(&state.entities_map[&e], Component::SecretWall)
                                        .unwrap();

                                let pos =
                                    get_component!(&state.entities_map[&e], Component::Position)
                                        .unwrap();

                                if groupb == group {
                                    state.remove_entity(*e);
                                    add_entity(create_floor(&mut state.entity_id_counter, &pos), state);
                                }
                            }
                        }
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