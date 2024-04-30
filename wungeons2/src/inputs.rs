use std::{
    collections::{HashMap, HashSet},
    io::{stdout, Write},
    process,
    thread::sleep,
    time::Duration,
};

use winit::keyboard::KeyCode;
// use device_query::{DeviceQuery, KeyCode};
use wurdle::{play, wurdle_words};

use crate::{
    components::{Component, Position, DIRECTIONS},
    create::{
        create_dialogue, create_floor, create_fog, create_item, create_revealed_floor,
        PLAYER_WALK_COOLDOWN,
    },
    entity::add_entity,
    event::{random_name, Event},
    get_component,
    items::{get_item_description, Item},
    state::State,
};

pub fn handle_inputs(state: &mut State, components: &[Component], key: Option<KeyCode>) {
    let entities = state
        .component_map
        .get(components)
        .unwrap_or(&HashSet::new())
        .clone();
    let player_entities = state
        .component_map
        .get(&vec![Component::Player])
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
    let directions: HashMap<KeyCode, Position> = HashMap::from_iter(DIRECTIONS);
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
    // let keys: Vec<KeyCode> = state.device_state.get_keys();
    // let mut pressed_key: Option<KeyCode> = None;
    let key_map: HashMap<KeyCode, char> = HashMap::from_iter([
        (KeyCode::KeyA, 'a'),
        (KeyCode::KeyB, 'b'),
        (KeyCode::KeyC, 'c'),
        (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'),
        (KeyCode::KeyF, 'f'),
        (KeyCode::KeyG, 'g'),
        (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'),
        (KeyCode::KeyJ, 'j'),
        (KeyCode::KeyK, 'k'),
        (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'),
        (KeyCode::KeyN, 'n'),
        (KeyCode::KeyO, 'o'),
        (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'),
        (KeyCode::KeyR, 'r'),
        (KeyCode::KeyS, 's'),
        (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'),
        (KeyCode::KeyV, 'v'),
        (KeyCode::KeyW, 'w'),
        (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'),
        (KeyCode::KeyZ, 'z'),
    ]);

    // if key.is_none() {
    //     state.last_pressed_keys.clear();
    //     return;
    // }
    let key = &key.unwrap();
    // if let Some(last_pressed_key) = &state.last_pressed_key {
    //     if *last_pressed_key == HashSet::from_iter(keys.clone()) && !dialogue_entities.is_empty() {
    //         // ignore if same key is pressed twice
    //         return;
    //     }
    // }
    // state.last_pressed_key = Some(HashSet::from_iter(keys.clone()));
    // if dialogue_entities.is_empty() {
    //     print!("\\e[?25l");
    //     stdout().flush().unwrap();
    // }
    // 'outer: for key in keys.iter() {
        // if *key == KeyCode::KeyF {

        //     dbg!(&keys, &dialogue_entities);
        //     process::exit(0);
        // }
        // if state.last_pressed_keys.contains(key) && !dialogue_entities.is_empty() {
        //     return;
        // } else if state.last_pressed_keys.contains(key) {
        //     match key {
        //         KeyCode::ArrowUp | KeyCode::ArrowRight | KeyCode::ArrowLeft | KeyCode::ArrowDown => {}
        //         _ => {
        //             return;
        //         }
        //     }
        // }
        // state.last_pressed_keys.insert(*key);

        // pressed_key = Some(*key);

        for (d, _) in &dialogue_entities {
            let (_, options) = get_component!(state.entities_map[d], Component::Dialogue).unwrap();
            if options.is_empty() {
                state.dialogue_input = "".to_string();
                state.remove_entity(*d);
                state.remove_all_by_component(Component::DialogueChar);
            }
            match key {
                KeyCode::Enter => {
                    let k: Option<(String, &Event)> = {
                        let mut r = None;
                        for (o, event) in &options {
                            if o == "" {
                                match event {
                                    Event::CreateName(_) => {
                                        if state.dialogue_input.trim().is_empty() {
                                            state.dialogue_input = random_name(&mut state.rng);
                                        }
                                    }
                                    _ => {}
                                }
                                if state.dialogue_input.trim().is_empty() {
                                   return;;
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
                KeyCode::Backspace => {
                    if !state.dialogue_input.is_empty() {
                        state.dialogue_input =
                            state.dialogue_input[0..state.dialogue_input.len() - 1].to_string();
                    }
                }
                _ => {
                    if key_map.contains_key(key) {
                        let mut st = key_map[key].to_string();
                        // if keys.contains(&KeyCode::ShiftLeft) || keys.contains(&KeyCode::ShiftRight) {
                        //     st = st.to_uppercase().to_string();
                        // }
                        state.dialogue_input = format!("{}{}", state.dialogue_input, st);
                    }
                }
            }

            return;;
        }

        // TODO needed?
        if !dialogue_entities.is_empty() {
            state.set_component(
                *player_entities.iter().next().unwrap(),
                Component::Cooldown(Some(0)),
            );
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
                KeyCode::KeyR => {
                    state.events.push(Event::Refresh);
                    state.events.push(Event::GameStart);
                }
                KeyCode::KeyF => {
                    state.fog_enabled = !state.fog_enabled;
                }
                KeyCode::KeyD => {
                    state.show_deck = !state.show_deck;
                }
                KeyCode::ArrowUp | KeyCode::ArrowRight | KeyCode::ArrowLeft | KeyCode::ArrowDown => {
                    if cooldown > 0 {
                        state.set_component(*e, Component::Cooldown(Some(cooldown - 1)));
                        // TODO
                        // continue;
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

                            sleep(Duration::from_millis(100));

                            let (won, attempts, word) = play(
                                tries,
                                HashSet::from_iter(state.available_letters.clone()),
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
                                            &mut state.rng,
                                            &mut state.entity_id_counter,
                                            &new_position,
                                            Some(drop),
                                            Some(0),
                                        ),
                                        state,
                                    );
                                }
                                if is_boss {
                                    state.floor += 1;
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
                            return;;
                        } else if item_entities.contains(&hit) {
                            let item =
                                get_component!(&state.entities_map[&hit], Component::Item).unwrap();

                            if state.entities_map[&hit]
                                .contains_component(&Component::Paywall(None))
                            {
                                let paywall =
                                    get_component!(&state.entities_map[&hit], Component::Paywall)
                                        .unwrap();

                                if paywall > 0 {
                                    let mut dialogue = vec![(
                                        format!("Buy {:?} for {}g?\n", item.clone(), paywall),
                                        None,
                                    )];
                                    dialogue.extend(get_item_description(&item));

                                    add_entity(
                                        create_dialogue(
                                            &mut state.entity_id_counter,
                                            dialogue,
                                            vec![
                                                (
                                                    "y".into(),
                                                    Event::BuyItem((hit, item, paywall, *e)),
                                                ),
                                                ("n".into(), Event::None),
                                            ],
                                            Position::ZERO,
                                        ),
                                        state,
                                    );
                                } else {
                                    let mut dialogue =
                                        vec![(format!("Pick up {:?}?\n", item.clone()), None)];
                                    dialogue.extend(get_item_description(&item));
                                    add_entity(
                                        create_dialogue(
                                            &mut state.entity_id_counter,
                                            dialogue,
                                            vec![
                                                (
                                                    "y".into(),
                                                    Event::BuyItem((hit, item, paywall, *e)),
                                                ),
                                                ("n".into(), Event::None),
                                            ],
                                            Position::ZERO,
                                        ),
                                        state,
                                    );
                                }
                            } else {
                                state.items.push(item);
                                state.remove_entity(hit);
                            }

                            return;;
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
                            return;;
                        } else if wall_entities.contains(&hit) {
                            return;;
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
                                        create_revealed_floor(&mut state.entity_id_counter, &pos),
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
                // KeyCode::KeySpace => {

                // }
                _ => {}
            }
        }
        // return;;
        // break 'outer;
    // }
    // if pressed_key.is_none() {
    //     state.last_pressed_key = None;
    //     // continue 'outer;
    // }
    // process::exit(code)
    // }
}
