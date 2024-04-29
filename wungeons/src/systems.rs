use std::process;

use crate::{
    components::{Component, Position},
    dialogue::dialogue,
    entity::{add_entity, new_entity},
    event::{game_events, Event},
    sight::sight,
    get_component,
    inputs::handle_inputs,
    render,
    state::State,
};

pub fn start_up(state: &mut State, _components: &[Component]) {

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
        (
            dialogue,
            vec![
                Component::Dialogue(None),
                Component::Position(None),
                Component::ZIndex(None),
            ],
            false,
        ),
        // (
        //     dummy,
        //     vec![Component::Dialogue(None), Component::Solid],
        //     false,
        // ),
        (
            sight,
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
