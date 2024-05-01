// use colored::Colorize;
// use device_query::{DeviceQuery, Keycode};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Component, Position},
    create::{create_dialogue, create_mystery_dialogue, create_player},
    effects::{
        get_all_modifiers_from_effects, get_effect_description, lowercase_first_letter,
        AllModifiers, Effect,
    },
    entity::{add_entity, new_entity},
    items::{get_item_effects, use_and_remove_item_on_pickup, Item},
    rooms::create_rooms,
    sight::ViewType,
    state::State,
};
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event {
    GameStart,
    Refresh,
    Welcome,
    None,
    ComponentChanged(Component),
    // a viewed b
    View(((usize, ViewType), (usize, ViewType))),

    // dialogue
    CreateName(Option<String>),

    // id, item, cost, player
    BuyItem((usize, Item, usize, usize)),
    CreateMystery(usize),
    UseEffect(Effect),
}

pub fn game_events(state: &mut State, _components: &[Component]) {
    state.events.reverse();
    // dbg!("{:?}", &state.events);
    // process::exit(0);
    let mut seen_wall_hint = false;
    loop {
        if let Some(event) = state.events.pop() {
            match event {
                Event::UseEffect(effect) => {
                    // let modifiers = use_item(&item, state);
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![(
                                format!(
                                    "You {}",
                                    lowercase_first_letter(&get_effect_description(&effect))
                                ),
                                None,
                            )],
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );
                    let mut mods = AllModifiers::default();
                    get_all_modifiers_from_effects(
                        &mut mods,
                        vec![effect],
                    );
                    state.apply_modifiers(&mut mods);
                }
                Event::CreateMystery(id) => {
                    state.remove_entity(id);
                    add_entity(
                        create_mystery_dialogue(&mut state.entity_id_counter, &mut state.rng),
                        state,
                    );
                }
                Event::BuyItem((e, item, cost, _player)) => {
                    if state.gold >= cost {
                        state.remove_entity(e);
                        state.gold -= cost;

                        if cost == 0 {
                            add_entity(
                                create_dialogue(
                                    &mut state.entity_id_counter,
                                    vec![(format!("You picked up the {:?}", item), None)],
                                    vec![],
                                    Position::ZERO,
                                ),
                                state,
                            );
                        } else {
                            add_entity(
                                create_dialogue(
                                    &mut state.entity_id_counter,
                                    vec![(
                                        format!("You bought the {:?} for {}g", item, cost),
                                        None,
                                    )],
                                    vec![],
                                    Position::ZERO,
                                ),
                                state,
                            );
                        }
                        if use_and_remove_item_on_pickup(&item) {
                            // let modifiers = use_item(&item, state);
                            let effects: Vec<Effect> = get_item_effects(&item);
                            get_all_modifiers_from_effects(&mut state.mods, effects);

                            // state.apply_modifiers(&modifiers);
                        } else {
                            state.items.push(item.clone());
                        }
                    } else {
                        add_entity(
                            create_dialogue(
                                &mut state.entity_id_counter,
                                vec![("You don't have enough gold!".into(), None)],
                                vec![],
                                Position::ZERO,
                            ),
                            state,
                        )
                    }
                }
                Event::View(((_a, ViewType::Player), (_b, ViewType::Minion))) => {
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![("You see a minion".to_string(), None)],
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );
                }
                Event::View(((_a, ViewType::Player), (_b, ViewType::SecretWall))) => {
                    if seen_wall_hint {
                        continue;
                    }
                    seen_wall_hint = true;
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![("You see a crack in the wall".to_string(), None)],
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );
                    state.remove_all_by_component(Component::SecretWallHint)
                }
                Event::View(_) => {}
                Event::ComponentChanged(c) => {
                    match c {
                        Component::StepCount(Some(count)) => {
                            // dbg!(count);
                            // process::exit(0);
                            if count == 5 {
                                // add_entity(
                                //     create_dialogue(
                                //         &mut state.entity_id_counter,
                                //         vec![("Step 5!".to_string(), None)],
                                //         vec![],
                                //         Position::ZERO,
                                //     ),
                                //     state,
                                // );
                            }
                        }
                        // Component::Minion(_) => todo!(),
                        // Component::Wall => todo!(),
                        // Component::SecretWall(_) => todo!(),
                        // Component::Room => todo!(),
                        // Component::Door => todo!(),
                        // Component::Position(_) => todo!(),
                        // Component::Render(_) => todo!(),
                        // Component::ZIndex(_) => todo!(),
                        // Component::Player => todo!(),
                        // Component::Drop(_) => todo!(),
                        // Component::Item(_) => todo!(),
                        // Component::Fog(_) => todo!(),
                        // Component::Solid => todo!(),
                        // Component::Dialogue(_) => todo!(),
                        // Component::DialogueChar => todo!(),
                        // Component::Activated(_) => todo!(),
                        // Component::Cooldown(_) => todo!(),
                        _ => {}
                    }
                }
                Event::CreateName(None) => todo!(),
                Event::CreateName(Some(name)) => {
                    state.name = name;
                    state.events.push(Event::Refresh);
                    state.events.push(Event::GameStart);
                }
                Event::None => {}
                Event::Welcome => {
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![
                                ("Welcome to ".to_string(), None),
                                ("WUNGEON".to_string(), Some((None, Some((255, 0, 0))))),
                                ("!\n\n(press any key to continue)".to_string(), None),
                            ],
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );

                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![("What is your name?\n\n(submit your name or press enter to be named)".to_string(), None)],
                            vec![("".to_string(), Event::CreateName(None))],
                            Position::ZERO,
                        ),
                        state,
                    );
                }
                Event::Refresh => {
                    state.refresh();
                }
                Event::GameStart => {
                    // remove all entities
                    let entity_ids: Vec<usize> = state.entities_map.keys().cloned().collect();
                    for id in entity_ids {
                        state.remove_entity(id);
                    }
                    state.grid_size.height = 30;
                    state.rooms.clear();
                    state.hallways.clear();
                    let (rooms, room_entities, hallways) = create_rooms(state);
                    state.rooms.extend(rooms);
                    state.hallways.extend(hallways);
                    for room_entity in room_entities {
                        add_entity(room_entity, state);
                    }
                    // create player

                    add_entity(
                        create_player(
                            &mut state.entity_id_counter,
                            state.rooms[0].0.center(&state.rooms[0].1),
                        ),
                        state,
                    );

                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            vec![(format!("Hello {}!", state.name).to_string(), None)],
                            vec![],
                            Position {
                                x: 0,
                                y: state.grid_size.height / 2,
                            },
                        ),
                        state,
                    );

                    add_entity(
                        new_entity(
                            &mut state.entity_id_counter,
                            vec![Component::StepCount(Some(0))],
                        ),
                        state,
                    );
                }
            }
        } else {
            return;
        }
    }
}

pub fn random_name(rng: &mut ThreadRng) -> String {
    let length = 3;
    // int length = new Scanner(System.in).nextInt();
    let vowels: Vec<char> = "aeiou".chars().collect();
    let consonants: Vec<char> = "bcdfghjklmnpqrstvwxyz".chars().collect();
    // Random gen = new Random();
    // let rng = &mut rand::thread_rng();
    // char[][] pairs = new char[length][2];
    // for (char[] pair : pairs) {
    // 	pair[0] = vowels.charAt(gen.nextInt(vowels.length()));
    // 	pair[1] = consonants.charAt(gen.nextInt(consonants.length()));
    // }
    let pairs = (0..length)
        .map(|_| {
            (
                vowels[rng.gen_range(0..vowels.len())],
                consonants[rng.gen_range(0..consonants.len())],
            )
        })
        .collect::<Vec<(char, char)>>();
    // System.out.println("Generated!");
    // System.out.print("The name is: ");
    // StringBuilder name = new StringBuilder();
    let mut name = vec![];
    for pair in &pairs {
        if rng.gen_bool(0.5) {
            name.push(pair.0);
            name.push(pair.1);
        } else {
            name.push(pair.0);
            name.push(pair.1);
        }
    }
    if rng.gen_bool(0.5) {
        if vowels.iter().any(|n| *n == name[name.len() - 1]) {
            name.push(consonants[rng.gen_range(0..consonants.len())]);
        } else {
            name.push(vowels[rng.gen_range(0..vowels.len())]);
        }
    }

    name.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}
