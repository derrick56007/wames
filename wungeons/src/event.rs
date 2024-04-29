use std::process;

use device_query::{DeviceQuery, Keycode};

use crate::{
    components::{Component, Position},
    create::{create_dialogue, create_player},
    entity::{add_entity, new_entity},
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
}

pub fn game_events(state: &mut State, _components: &[Component]) {
    state.events.reverse();
    // dbg!("{:?}", &state.events);
    // process::exit(0);
    loop {
        if let Some(event) = state.events.pop() {
            match event {
                Event::View(((a, ViewType::Player), (b, ViewType::Minion))) => {
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            format!("You see a minion"),
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );
                },
                Event::View(((a, ViewType::Player), (b, ViewType::SecretWall))) => {
                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            format!("You see a crack in the wall"),
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );
                }
                Event::View(_) => {}
                Event::ComponentChanged(c) => {
                    match c {
                        Component::StepCount(Some(count)) => {
                            // dbg!(count);
                            // process::exit(0);
                            if count == 5 {
                                add_entity(
                                    create_dialogue(
                                        &mut state.entity_id_counter,
                                        "Step 5!".to_string(),
                                        vec![],
                                        Position::ZERO,
                                    ),
                                    state,
                                );
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
                            "Welcome to the game!".to_string(),
                            vec![],
                            Position::ZERO,
                        ),
                        state,
                    );

                    add_entity(
                        create_dialogue(
                            &mut state.entity_id_counter,
                            "What is your name?".to_string(),
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
                            format!("Hello {}!", state.name).to_string(),
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

