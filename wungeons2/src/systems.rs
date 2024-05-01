use crate::{
    components::Component,
    dialogue::dialogue,
    event::{game_events, Event},
    sight::sight,
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
            dummy,
            vec![
                Component::Position(None),
                Component::Render(None),
                Component::ZIndex(None),
            ],
            false,
        ),
        (
            dummy,
            vec![Component::Invisible(None), Component::Position(None)],
            false,
        ),
    ]
}

fn dummy(_state: &mut State, _components: &[Component]) {}
