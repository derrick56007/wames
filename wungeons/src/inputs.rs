use std::{
    collections::{HashMap, HashSet},
    process,
    thread::sleep,
    time::Duration,
};

use device_query::{DeviceQuery, Keycode};
use wurdle::{play, wurdle_words};

use crate::{
    components::{Component, Item, Position, DIRECTIONS}, create::{create_floor, create_fog, create_item, PLAYER_WALK_COOLDOWN}, entity::add_entity, event::Event, get_component, state::State
};

pub fn handle_inputs(state: &mut State, components: &[Component]) {
    let entities = state
        .component_map
        .get(components)
        .unwrap_or(&HashSet::new())
        .clone();
    let minion_entities = state
        .component_map
        .get(&vec![Component::Minion(None)])
        .unwrap_or(&HashSet::new())
        .clone();
    let item_entities = state
        .component_map
        .get(&vec![Component::Item(None)])
        .unwrap_or(&HashSet::new())
        .clone();
    let door_entities = state
        .component_map
        .get(&vec![Component::Door])
        .unwrap_or(&HashSet::new())
        .clone();
    let wall_entities = state
        .component_map
        .get(&vec![Component::Wall])
        .unwrap_or(&HashSet::new())
        .clone();
    let dialogue_entities = state
        .component_map
        .get(&vec![Component::Dialogue(None)])
        .unwrap_or(&HashSet::new())
        .clone();
    let mut dialogue_entities = dialogue_entities
        .iter()
        .map(|e| {
            (
                *e,
                get_component!(state.entities_map[e], Component::ZIndex).unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    let mut step_count_entities = state
        .component_map
        .get(&vec![Component::StepCount(None)])
        .unwrap_or(&HashSet::new())
        .clone();

    dialogue_entities.sort_by(|a, b| a.1.cmp(&b.1));

    let secret_wall_entities = state
        .component_map
        .get(&vec![Component::SecretWall(None)])
        .unwrap_or(&HashSet::new())
        .clone();

    // dbg!(&state.component_map);

    // let entity_ids: Vec<usize> = state.entities_map.keys().cloned().collect();
    let directions: HashMap<Keycode, Position> = HashMap::from_iter(DIRECTIONS);
    let other_positions = state
        .component_map
        .get(&vec![Component::Position(None)])
        .unwrap_or(&HashSet::new())
        .clone()
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
    // let mut pressed_key: Option<Keycode> = None;
    let key_map: HashMap<Keycode, char> = HashMap::from_iter([
        (Keycode::A, 'a'),
        (Keycode::B, 'b'),
        (Keycode::C, 'c'),
        (Keycode::D, 'd'),
        (Keycode::E, 'e'),
        (Keycode::F, 'f'),
        (Keycode::G, 'g'),
        (Keycode::H, 'h'),
        (Keycode::I, 'i'),
        (Keycode::J, 'j'),
        (Keycode::K, 'k'),
        (Keycode::L, 'l'),
        (Keycode::M, 'm'),
        (Keycode::N, 'n'),
        (Keycode::O, 'o'),
        (Keycode::P, 'p'),
        (Keycode::Q, 'q'),
        (Keycode::R, 'r'),
        (Keycode::S, 's'),
        (Keycode::T, 't'),
        (Keycode::U, 'u'),
        (Keycode::V, 'v'),
        (Keycode::W, 'w'),
        (Keycode::X, 'x'),
        (Keycode::Y, 'y'),
        (Keycode::Z, 'z'),
    ]);

    if keys.is_empty() {
        state.last_pressed_keys.clear();
    }
    // if let Some(last_pressed_key) = &state.last_pressed_key {
    //     if *last_pressed_key == HashSet::from_iter(keys.clone()) && !dialogue_entities.is_empty() {
    //         // ignore if same key is pressed twice
    //         return;
    //     }
    // }
    // state.last_pressed_key = Some(HashSet::from_iter(keys.clone()));

    'outer: for key in keys.iter() {
        // if *key == Keycode::F {

        //     dbg!(&keys, &dialogue_entities);
        //     process::exit(0);
        // }
        if state.last_pressed_keys.contains(key) && !dialogue_entities.is_empty() {
            continue;
        }
        else if state.last_pressed_keys.contains(key) {
            match key {

                Keycode::Up | Keycode::Right | Keycode::Left | Keycode::Down => {

                }
                _ => {
                    continue;
                }
            }
        }
        state.last_pressed_keys.insert(*key);

        // pressed_key = Some(*key);

        for (d, _) in &dialogue_entities {
            let (_, options) = get_component!(state.entities_map[d], Component::Dialogue).unwrap();
            // if options.is_empty() {
            //     state.dialogue_input = "".to_string();
            //     state.remove_entity(*d);
            //     state.remove_all_by_component(Component::DialogueChar);
            // }
            match key {
                Keycode::Enter => {
                    let k: Option<(String, &Event)> = {
                        let mut r = None;
                        for (o, event) in &options {
                            if o == "" {
                                if state.dialogue_input.trim().is_empty() {
                                    break 'outer;
                                }
                                r = Some((state.dialogue_input.clone(), event));
                            } else if state.dialogue_input == *o {
                                r = Some((state.dialogue_input.clone(), event));
                            }
                        }
                        r
                    };

                    if options.is_empty() {
                        state.dialogue_input = "".to_string();
                        state.remove_entity(*d);
                        state.remove_all_by_component(Component::DialogueChar);
                    } else if let Some((o, event)) = k {
                        let event = match event {
                            Event::CreateName(None) => Event::CreateName(Some(o)),
                            any => any.clone(),
                        };
                        state.events.push(event);
                        state.remove_entity(*d);
                        state.remove_all_by_component(Component::DialogueChar);
                    }

                    state.dialogue_input = "".to_string();
                }
                Keycode::Backspace => {
                    if !state.dialogue_input.is_empty() {
                        state.dialogue_input =
                            state.dialogue_input[0..state.dialogue_input.len() - 1].to_string();
                    }
                }
                _ => {
                    if key_map.contains_key(key) {
                        let mut st = key_map[key].to_string();
                        if keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift) {
                            st = st.to_uppercase().to_string();
                        }
                        state.dialogue_input = format!("{}{}", state.dialogue_input, st);
                    }
                }
            }

            break 'outer;
        }

        for e in entities.iter() {
            let entity = &e;
            let cooldown = get_component!(&state.entities_map[&e], Component::Cooldown).unwrap();

            state.set_component(*e, Component::Cooldown(Some(PLAYER_WALK_COOLDOWN)));

            let position: Position =
                get_component!(state.entities_map[entity], Component::Position).unwrap();

            
            // match key {
            //      => {
            //         // break;
            //     }
            //     _ => {
            //         if state.last_pressed_keys.contains(key) {
            //             continue 'outer;
            //         }
            //     }
            // }
            match key {
                Keycode::R => {
                    state.events.push(Event::Refresh);
                    state.events.push(Event::GameStart);
                }
                Keycode::F => {
                    state.fog_enabled = !state.fog_enabled;
                }
                Keycode::Up | Keycode::Right | Keycode::Left | Keycode::Down => {
                    if cooldown > 0 {
                        state.set_component(*e, Component::Cooldown(Some(cooldown - 1)));
                        continue;
                    }
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
                            // let render: char =
                            //     get_component!(&state.entities_map[&hit], Component::Render)
                            //         .unwrap();
                            let (is_boss, letter) =
                                get_component!(&state.entities_map[&hit], Component::Minion)
                                    .unwrap();
                            state.add_letter(letter);

                            let tries = 6;
                            let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
                                .split('\n')
                                .map(|s| s.to_uppercase())
                                .collect();

                            // sleep(Duration::from_millis(100));

                            let (won, attempts, word) = play(
                                tries,
                                state.available_letters.clone(),
                                words_vec,
                                Some(letter),
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
                                    add_entity(
                                        create_floor(&mut state.entity_id_counter, &pos),
                                        state,
                                    );
                                    add_entity(
                                        create_fog(&mut state.entity_id_counter, &pos),
                                        state,
                                    );
                                }
                            }
                        }
                        // break;
                    }

                    state.set_component(*e, Component::Position(Some(new_position.clone())));

                    for s in &step_count_entities {
                        // let e = state.entities_map[&s];
                        let step_count =
                            get_component!(state.entities_map[&s], Component::StepCount).unwrap();
                        state.set_component(*s, Component::StepCount(Some(step_count + 1)));
                    }
                    step_count_entities.clear();
                    // state.step_counter += 1;
                    // }
                }
                // Keycode::Space => {

                // }
                _ => {}
            }
        }
        break;
        // break 'outer;
    }
    // if pressed_key.is_none() {
    //     state.last_pressed_key = None;
    //     // continue 'outer;
    // }
    // process::exit(code)
    // }
}
