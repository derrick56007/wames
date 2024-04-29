use crate::{components::{Component, Position}, entity::{add_entity, new_entity}, event::Event, get_component, state::State};


pub fn dialogue(state: &mut State, components: &[Component]) {
    let entities = state.get_entities(components).clone();
    let mut entities = entities
        .iter()
        .map(|e| {
            (
                *e,
                get_component!(state.entities_map[e], Component::ZIndex).unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    entities.sort_by(|a, b| a.1.cmp(&b.1));

    for (e, z) in &entities {
        let activated = get_component!(state.entities_map[e], Component::Activated).unwrap();
        if activated {
            break;
        }
        let dialogue: (String, Vec<(String, Event)>) =
            get_component!(state.entities_map[e], Component::Dialogue).unwrap();
        let pos = get_component!(state.entities_map[e], Component::Position).unwrap();

        // let y = 0;
        for (x, c) in dialogue.0.chars().enumerate() {
            add_entity(
                new_entity(
                    &mut state.entity_id_counter,
                    vec![
                        Component::Position(Some(Position {
                            x: pos.x + x as isize,
                            y: pos.y,
                        })),
                        Component::ZIndex(Some(*z)),
                        Component::Render(Some(c)),
                        Component::DialogueChar,
                    ],
                ),
                state,
            );
        }
        state.set_component(*e, Component::Activated(Some(true)));
        break;
    }
}