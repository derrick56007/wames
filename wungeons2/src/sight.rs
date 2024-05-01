use std::collections::HashSet;

use components::Position;
// use device_query::{DeviceQuery, Keycode};

use event::Event;

use render::bresenham;
// use rooms::{create_floor, create_item};

use crate::{
    components::{self, Component},
    create::BLACK,
    entity::{self},
    event, get_component, render,
    state::State,
};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum ViewType {
    Player,
    Minion,
    SecretWall,
}

pub fn sight(state: &mut State, components: &[Component]) {
    let fog_entities = state.get_entities(components).clone();
    let viewers = state.get_entities(&[Component::ViewDistance(None)]).clone();
    let viewables = state.get_entities(&[Component::Viewable(None)]).clone();

    let solid_positions = state
        .get_entities(&[Component::Position(None), Component::Solid])
        .clone();
    let solid_positions = solid_positions
        .iter()
        .map(|e| get_component!(state.entities_map[e], Component::Position).unwrap())
        .collect::<Vec<Position>>();

    for f in fog_entities.iter() {
        let visited = get_component!(state.entities_map[f], Component::Fog).unwrap();
        if state.fog_enabled {
            match visited {
                true => {
                    state.set_component(
                        *f,
                        Component::Render(Some(("█".to_string(), (55, 55, 55)))),
                    );
                }
                _ => {
                    state.set_component(*f, Component::Render(Some(("█".to_string(), BLACK))));
                }
            }
        } else {
            state.set_component(*f, Component::Render(None));
        }
    }

    for viewer in viewers {
        let view_distance =
            get_component!(state.entities_map[&viewer], Component::ViewDistance).unwrap();

        let viewer_pos = get_component!(state.entities_map[&viewer], Component::Position).unwrap();
        let mut seen_positions = HashSet::new();
        for y in 0..state.grid_size.height {
            for x in 0..state.grid_size.width {
                let pos = Position { x, y };
                'inner:
                for p in bresenham(&viewer_pos, &pos)
                    .iter()
                    .copied()
                    .take(view_distance)
                {
                    if solid_positions.contains(&p) {
                        break 'inner;
                    }
                    seen_positions.insert(p);
                }
            }
        }

        for viewable in &viewables {
            if viewer != *viewable {
                let viewable_pos =
                    get_component!(state.entities_map[viewable], Component::Position).unwrap();
                let (viewable_type, mut viewable_viewed_by) =
                    get_component!(state.entities_map[viewable], Component::Viewable).unwrap();
                if seen_positions.contains(&viewable_pos) {
                    let (viewer_type, mut viewer_viewed_by) =
                        get_component!(state.entities_map[&viewer], Component::Viewable).unwrap();
                    if !viewable_viewed_by.contains(&viewer) {
                        state.events.push(Event::View((
                            (viewer, viewer_type.clone()),
                            (*viewable, viewable_type.clone()),
                        )));
                        viewable_viewed_by.push(viewer);
                        state.set_component(
                            *viewable,
                            Component::Viewable(Some((
                                viewable_type.clone(),
                                viewable_viewed_by.clone(),
                            ))),
                        );

                        state.events.push(Event::View((
                            (*viewable, viewable_type.clone()),
                            (viewer, viewer_type.clone()),
                        )));
                        viewer_viewed_by.push(*viewable);
                        state.set_component(
                            viewer,
                            Component::Viewable(Some((viewer_type, viewer_viewed_by))),
                        );
                    }
                }
            }
        }

        if !state.fog_enabled {
            continue;
        }
        if state.entities_map[&viewer].contains_component(&Component::AffectsFog) {
            for f in fog_entities.iter() {
                let fog_pos = get_component!(state.entities_map[f], Component::Position).unwrap();
                if seen_positions.contains(&fog_pos) {
                    let visited = get_component!(state.entities_map[f], Component::Fog).unwrap();
                    match visited {
                        // components::FogState::Dark(true) => {
                        //     state.entities_map.get_mut(f).unwrap().set_component(Component::Render(Some('*')));
                        // },
                        false => {
                            state.set_component(*f, Component::Fog(Some(true)));
                        }
                        _ => {} // components::FogState::Clear => {

                                // },
                    }
                    state.set_component(*f, Component::Render(None));
                }
            }
        }
    }
}
