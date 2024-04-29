use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    process,
    thread::sleep,
    time::{Duration, SystemTime},
};

use components::{Item, Position, DIRECTIONS};
use device_query::{DeviceQuery, Keycode};
use entity::new_entity;
use event::Event;

use render::bresenham;
// use rooms::{create_floor, create_item};
use wurdle::{play, wurdle_words};

use crate::{
    components::{self, Component, Rect},
    entity::{self, add_entity},
    event, get_component,
    inputs::handle_inputs,
    render,
    rooms::{self, create_rooms},
    state::State,
};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum ViewType {
    Player,
    Minion,
    SecretWall
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
        let fog = get_component!(state.entities_map[f], Component::Fog).unwrap();
        if state.fog_enabled {
            match fog {
                true => {
                    state.set_component(*f, Component::Render(Some('░')));
                }
                _ => {
                    state.set_component(*f, Component::Render(Some(' ')));
                }
            }
        } else {
            state.set_component(*f, Component::Render(None));
        }
    }
    if !state.fog_enabled {
        return;
    }
    for viewer in viewers {
        let view_distance =
            get_component!(state.entities_map[&viewer], Component::ViewDistance).unwrap();

        let viewer_pos = get_component!(state.entities_map[&viewer], Component::Position).unwrap();
        let mut seen_positions = HashSet::new();
        for (rect, pos, _) in &state.rooms {
            for x in pos.x..pos.x + rect.width {
                for p in bresenham(&viewer_pos, &Position { x, y: pos.y })
                    .iter()
                    .copied()
                    .take(view_distance)
                {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    seen_positions.insert(p);
                }
                for p in bresenham(
                    &viewer_pos,
                    &Position {
                        x,
                        y: pos.y + rect.height,
                    },
                )
                .iter()
                .copied()
                .take(view_distance)
                {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    seen_positions.insert(p);
                }
            }

            for y in pos.y..pos.y + rect.height {
                for p in bresenham(&viewer_pos, &Position { x: pos.x, y })
                    .iter()
                    .copied()
                    .take(view_distance)
                {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    seen_positions.insert(p);
                }
                for p in bresenham(
                    &viewer_pos,
                    &Position {
                        x: pos.x + rect.width,
                        y,
                    },
                )
                .iter()
                .copied()
                .take(view_distance)
                {
                    if solid_positions.contains(&p) {
                        break;
                    }
                    seen_positions.insert(p);
                }
            }
        }
        for (pos1, pos2) in &state.hallways {
            for p in bresenham(&viewer_pos, pos1)
                .iter()
                .copied()
                .take(view_distance)
            {
                if solid_positions.contains(&p) {
                    break;
                }
                seen_positions.insert(p);
            }
            for p in bresenham(&viewer_pos, pos2)
                .iter()
                .copied()
                .take(view_distance)
            {
                if solid_positions.contains(&p) {
                    break;
                }
                seen_positions.insert(p);
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
                            (viewer.clone(), viewer_type.clone()),
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
                } else {
                }
            }
        }
    }
}
