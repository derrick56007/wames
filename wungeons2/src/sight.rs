use std::collections::{HashMap, HashSet};

use components::Position;
// use device_query::{DeviceQuery, Keycode};

use event::Event;

use fnv::{FnvHashMap, FnvHashSet};
use render::bresenham;
// use rooms::{create_floor, create_item};

use crate::{
    components::{self, Component},
    entity,
    event, get_component, render,
    state::State,
};


use crate::{
    colors::*
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
    let invisible_entities = state
        .get_entities(&[Component::Hidden(None), Component::Position(None)])
        .clone();
    let mut invisible_positions: FnvHashMap<Position, Vec<usize>> = FnvHashMap::default();


    for e in invisible_entities{
        let p =  get_component!(state.entities_map[&e], Component::Position).unwrap();

        if !invisible_positions.contains_key(&p) {
            invisible_positions.insert(p, vec![]);
        }
        invisible_positions.get_mut(&p).unwrap().push(e);

    }
    
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
                    // let (render_c, _) =  get_component!(state.entities_map[f], Component::Render).unwrap();
                    state.set_component(
                        *f,
                        Component::RenderFg(Some(("█".to_string(), REVEALED_FOG_COLOR, false))),
                    );
                }
                _ => {
                    // state.set_component(*f, Component::Render(Some(("█".to_string(), BLACK))));
                }
            }
        } else {
            state.set_component(*f, Component::RenderFg(None));
        }
    }

    // if !state.fog_enabled {
    //     for (_, p) in &invisible_positions {
    //         state.set_component(*p, Component::Invisible(Some(false)))
    //     }
    // }

    for viewer in viewers {
        let view_distance =
            get_component!(state.entities_map[&viewer], Component::ViewDistance).unwrap();

        let viewer_pos = get_component!(state.entities_map[&viewer], Component::Position).unwrap();
        let mut seen_positions = FnvHashSet::default();
        let a= -(view_distance as isize);
        let b = view_distance as isize;
        for y in a..b {
            for x in a..b {
                if x > a && x < b - 1 && y > a && y < b - 1 {
                    continue;
                }
                let pos = viewer_pos + &Position { x, y };

                'inner: for p in bresenham(&viewer_pos, &pos)
                    .iter()
                    .copied()
                    .take(view_distance)
                {
                    if invisible_positions.contains_key(&p)
                        && state.entities_map[&viewer].contains_component(&Component::AffectsFog)
                    {
                        for e in &invisible_positions[&p] {

                            state.set_component(
                                *e,
                                Component::Hidden(Some(false)),
                            )
                        }
                    }
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
                    state.set_component(*f, Component::RenderFg(None));
                }
            }
        }
    }
}
