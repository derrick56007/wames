use crate::{
    components::{Component, Position},
    create::WHITE,
    entity::{add_entity, new_entity},
    event::Event,
    get_component,
    state::State,
};

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
        let dialogue: (
            Vec<(String, Option<(Option<(u8, u8, u8)>, Option<(u8, u8, u8)>)>)>,
            Vec<(String, Event)>,
        ) = get_component!(state.entities_map[e], Component::Dialogue).unwrap();
        let pos = get_component!(state.entities_map[e], Component::Position).unwrap();

        // let y = 0;
        let mut x = 0;
        let mut y = 0;
        for (line, color) in dialogue.0 {
            for c in line.chars() {
                if c == '\n' {
                    x = 0;
                    y += 1;
                    continue;
                }
                let render_c = match color {
                    Some((Some(color), None)) => Component::Render(Some((c, Some(color), None))),
                    Some((None, Some(color))) => Component::Render(Some((c, None, Some(color)))),
                    None => Component::Render(Some((c, None, Some(WHITE)))),
                    _ => todo!(),
                };
                add_entity(
                    new_entity(
                        &mut state.entity_id_counter,
                        vec![
                            Component::Position(Some(Position {
                                x: pos.x + x,
                                y: pos.y + y,
                            })),
                            Component::ZIndex(Some(*z)),
                            render_c,
                            Component::DialogueChar,
                        ],
                    ),
                    state,
                );
                x += 1;
            }
        }
        state.set_component(*e, Component::Activated(Some(true)));
        break;
    }
}
