use std::process;

use crate::{
    components::{Component, Position},
    entity::{add_entity, new_entity},
    event::{game_events, Event},
    fog::calculate_fog,
    get_component,
    inputs::handle_inputs,
    render,
    state::State,
};

pub fn start_up(state: &mut State, _components: &[Component]) {
    state.events.push(Event::Refresh);
    state.events.push(Event::GameStart);
    state.events.push(Event::Welcome);
}

pub fn get_systems() -> Vec<(fn(&mut State, &[Component]), Vec<Component>, bool)> {
    vec![
        (start_up, vec![], true),
        (game_events, vec![], false),
        (
            dummy,
            vec![Component::Position(None), Component::Solid],
            false,
        ),
        (dialogue, vec![Component::Dialogue(None)], false),
        // (
        //     dummy,
        //     vec![Component::Dialogue(None), Component::Solid],
        //     false,
        // ),
        (
            calculate_fog,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
                Component::Fog(None),
            ],
            false,
        ),
        (
            render,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
            ],
            false,
        ),
        (handle_inputs, vec![Component::Player], false),
        // (collisions, vec![], false),
    ]
}

fn dummy(state: &mut State, components: &[Component]) {}
fn dialogue(state: &mut State, components: &[Component]) {
    let entities = state.get_entities(components).clone();
    // let existing_char_entities = state.get_entities(&vec![Component::DialogueChar]).clone();
    // state.remove_all_by_component(Component::DialogueChar);
    // dbg!(&state.component_map.keys(), &entities);
    // process::exit(0);
    // return;
    for e in &entities {
        let activated = get_component!(state.entities_map[e], Component::Activated).unwrap();
        if activated {
            break;
        }
        let dialogue = get_component!(state.entities_map[e], Component::Dialogue).unwrap();

        // let y = 0;
        for (x, c) in dialogue.0.chars().enumerate() {
            add_entity(new_entity(
                &mut state.entity_id_counter,
                vec![
                    Component::Position(Some(Position {
                        x: x as isize,
                        y: x as isize / state.grid_size.width,
                    })),
                    Component::ZIndex(Some(6)),
                    Component::Render(Some(c)),
                    Component::DialogueChar,
                ],
            ), state);
        }
        let entity = &mut state.entities_map.get_mut(e).unwrap();
        entity.set_component(Component::Activated(Some(true)));
        break;
    }
}
