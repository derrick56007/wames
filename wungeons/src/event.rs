use std::process;

use device_query::{DeviceQuery, Keycode};

use crate::{
    components::Component,
    entity::{add_entity, new_entity},
    rooms::create_rooms,
    state::State,
};
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event {
    GameStart,
    Refresh,
    Welcome,
    None,
    CreateName(Option<String>),
}

pub fn game_events(state: &mut State, _components: &[Component]) {
    state.events.reverse();
    // dbg!("{:?}", &state.events);
    // process::exit(0);
    loop {
        if let Some(event) = state.events.pop() {
            match event {
                Event::CreateName(None) => todo!(),
                Event::CreateName(Some(name)) => {
                    add_entity(
                        new_entity(
                            &mut state.entity_id_counter,
                            vec![
                                Component::Activated(Some(false)),
                                Component::Dialogue(Some((
                                    format!("Hello {name}!").to_string(),
                                    vec![],
                                ))),
                            ],
                        ),
                        state,
                    );
                }
                Event::None => {}
                Event::Welcome => {
                    add_entity(
                        new_entity(
                            &mut state.entity_id_counter,
                            vec![
                                Component::Activated(Some(false)),
                                Component::Dialogue(Some((
                                    "Welcome to the game!".to_string(),
                                    vec![],
                                ))),
                            ],
                        ),
                        state,
                    );
                    add_entity(
                        new_entity(
                            &mut state.entity_id_counter,
                            vec![
                                Component::Activated(Some(false)),
                                Component::Dialogue(Some((
                                    "What is your name?".to_string(),
                                    vec![("".to_string(), Event::CreateName(None))],
                                ))),
                            ],
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
                    let entity = new_entity(
                        &mut state.entity_id_counter,
                        vec![
                            Component::Position(Some(state.rooms[0].0.center(&state.rooms[0].1))),
                            Component::Render(Some('@')),
                            Component::ZIndex(Some(5)),
                            Component::Player,
                            Component::Cooldown(Some(PLAYER_WALK_COOLDOWN))
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

pub const PLAYER_WALK_COOLDOWN: usize = 2;