use std::{collections::HashSet, ops::Range};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{
        get_item_char, intersects, line_rect, Component, Item, Position, Rect, DIAGONAL_DIRECTIONS,
        DIRECTIONS,
    },
    entity::{new_entity, Entity},
    render::bresenham,
    state::State,
};

const BIG_ROOM_WIDTH_RANGE: Range<usize> = 11..15;
const BIG_ROOM_HEIGHT_RANGE: Range<usize> = 7..11;

const ROOM_WIDTH_RANGE: Range<usize> = 6..9;
const ROOM_HEIGHT_RANGE: Range<usize> = 4..6;

fn generate_random_point_in_circle(rng: &mut ThreadRng, radius: isize) -> (isize, isize) {
    let random_angle = rng.gen::<f64>() * std::f64::consts::PI * 2.0;
    let random_radius = (radius as f64) * random_angle.sqrt();
    let x = random_radius * random_angle.cos();
    let y = random_radius * random_angle.sin();
    (x as isize, y as isize)
}

fn find_nearest_free_space(
    rooms: &[(Rect, Position, RoomType)],
    room_index: usize,
    // grid_size: &Rect,
    rng: &mut ThreadRng,
) -> Option<Position> {
    let (room, pos, _) = &rooms[room_index];

    for (i, (other_room, other_pos, _)) in rooms.iter().enumerate() {
        if i != room_index && intersects(pos, room, other_pos, other_room) {
            return Some(Position {
                x: pos.x + rng.gen_range(-4..5),
                y: pos.y + rng.gen_range(-1..2),
            });
        }
    }

    None
}

fn generate_rand_position(rng: &mut ThreadRng, grid_size: &Rect, rect: &Rect) -> Position {
    let random_circle_point = generate_random_point_in_circle(rng, 5);

    Position {
        x: (grid_size.width / 2 + random_circle_point.0 - rect.width / 2) as isize,
        y: (grid_size.height / 2 + random_circle_point.1 - rect.height / 2) as isize,
    }
}

pub fn resolve_collisions(
    rooms: &mut Vec<(Rect, Position, RoomType)>,
    grid_size: &Rect,
    rng: &mut ThreadRng,
    start: usize,
) {
    for room_index in start..rooms.len() {
        'inner: loop {
            if let Some(new_pos) = find_nearest_free_space(rooms, room_index, rng) {
                let (room_rect, _, _) = &rooms[room_index];
                // dbg!(&room_index, &new_pos);
                if new_pos.x >= 0
                    && new_pos.x + room_rect.width < grid_size.width
                    && new_pos.y >= 0
                    && new_pos.y + room_rect.height < grid_size.height
                {
                    rooms[room_index].1 = new_pos;
                } else {
                    rooms[room_index].1 = generate_rand_position(rng, grid_size, room_rect);
                }

                continue 'inner;
            }
            break 'inner;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    Big,
    Boss,
    Regular,
    Starting,
    Secret,
}

pub fn create_rooms(
    // rng: &mut ThreadRng,
    // grid_size: &Rect,
    // entity_id_counter: &mut usize,
    state: &mut State,
) -> (Vec<(Rect, Position, RoomType)>, Vec<Entity>) {
    let mut rooms: Vec<(Rect, Position, RoomType)> = vec![];
    let num_big_rooms = 4;
    let num_small_rooms = 5;
    let num_rooms = 5;
    let mut big_lines: Vec<(Position, Position)> = vec![];

    let grid_size: &Rect = &state.grid_size.clone();
    let mut minion_letters = state.get_unchosen_letters_if_possible(num_big_rooms);
    let rng: &mut ThreadRng = &mut state.rng;

    let entity_id_counter: &mut usize = &mut state.entity_id_counter;

    for _ in 0..num_small_rooms {
        let rect = Rect {
            width: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((rect, pos, RoomType::Regular));
    }
    rooms[0].2 = RoomType::Starting;

    resolve_collisions(&mut rooms, grid_size, rng, 0);

    for i in 0..num_big_rooms {
        let rect = Rect {
            width: rng.gen_range(BIG_ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(BIG_ROOM_HEIGHT_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((
            rect,
            pos,
            if i == 0 {
                RoomType::Boss
            } else {
                RoomType::Big
            },
        ));
    }

    resolve_collisions(&mut rooms, grid_size, rng, num_small_rooms);

    for _ in 0..num_rooms {
        let rect = Rect {
            width: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(ROOM_HEIGHT_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((rect, pos, RoomType::Regular));
    }

    // resolve room positions
    resolve_collisions(&mut rooms, grid_size, rng, num_small_rooms + num_big_rooms);

    for i in 1..num_big_rooms + num_small_rooms {
        let (last_rect, last_pos, _) = &rooms[i - 1];
        let (rect, pos, _) = &rooms[i];
        big_lines.push((last_rect.center(last_pos), rect.center(pos)))
    }

    let mut final_rooms = vec![];

    for (i, (rect, pos, is_big_room)) in rooms.iter().enumerate() {
        if i >= num_big_rooms + num_small_rooms
            && !big_lines.iter().any(|(pos1, pos2)| {
                line_rect(
                    pos1.x as f64,
                    pos1.y as f64,
                    pos2.x as f64,
                    pos2.y as f64,
                    pos.x as f64,
                    pos.y as f64,
                    rect.width as f64,
                    rect.height as f64,
                )
            })
        {
            continue;
        }
        final_rooms.push((rect, pos, is_big_room));
    }
    let len = final_rooms.len();
    for i in 0..len {
        if final_rooms[i].2 == &RoomType::Regular {
            final_rooms[len - 1].2 = &RoomType::Secret;
            break;
        }
    }


    let mut hallways: Vec<(Position, Position)> = vec![];
    for i in 1..final_rooms.len() {
        let (last_rect, last_pos, _last_room_type) = &final_rooms[i - 1];
        let (rect, pos, _room_type) = &final_rooms[i];
        let last_center = last_rect.center(last_pos);
        let center = rect.center(pos);

        // if *last_room_type != &RoomType::Secret && *room_type != &RoomType::Secret {

        hallways.push((
            last_center.clone(),
            Position {
                x: last_center.x,
                y: center.y,
            },
        ));

        hallways.push((
            center.clone(),
            Position {
                x: last_center.x,
                y: center.y,
            },
        ));
        // }
    }
    let mut wall_positions: HashSet<Position> = HashSet::new();
    let mut secret_wall_positions: HashSet<Position> = HashSet::new();

    let mut fog_positions: HashSet<Position> = HashSet::new();

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            wall_positions.insert(Position { x, y });
            fog_positions.insert(Position { x, y });
        }
    }
    let mut entities = vec![];

    let mut secret_hallways: Vec<(Position, Position)> = vec![];

    // dig hallways
    for (pos1, pos2) in hallways {
        let (rect, pos, _) = final_rooms[len - 1];
        if !line_rect(
            pos1.x as f64,
            pos1.y as f64,
            pos2.x as f64,
            pos2.y as f64,
            pos.x as f64,
            pos.y as f64,
            rect.width as f64,
            rect.height as f64,
        ) {
            for line_pos in bresenham(&pos1, &pos2) {
                wall_positions.remove(&line_pos);
                entities.push(create_floor(entity_id_counter, &line_pos));
            }
        }
        secret_hallways.push((pos1, pos2));
    }

    // dig out rooms
    for (rect, pos, _room_type) in final_rooms.iter() {
        for x in pos.x + 1..pos.x + rect.width {
            for y in pos.y + 1..pos.y + rect.height {
                wall_positions.remove(&Position { x, y });
                entities.push(create_floor(entity_id_counter, &Position { x, y }));
            }
        }
    }

    for (pos1, pos2) in secret_hallways {
        for line_pos in bresenham(&pos1, &pos2) {
            if wall_positions.contains(&line_pos) {
                wall_positions.remove(&line_pos);
                secret_wall_positions.insert(line_pos);
            }
        }
    }

    let mut spawned_key = false;

    // create doors
    for (rect, pos, room_type) in final_rooms.iter() {
        // break;
        let room_type = **room_type;
        if room_type == RoomType::Boss {
            let boss_position = rect.center(pos);

            for (_, d) in DIRECTIONS {
                entities.push(create_door(entity_id_counter, &(&boss_position + &d)));
            }
            for (_, d) in DIAGONAL_DIRECTIONS {
                entities.push(create_door(entity_id_counter, &(&boss_position + &d)));
            }

            entities.push(create_minion(
                entity_id_counter,
                &boss_position,
                minion_letters.pop().unwrap(),
                true,
                false,
            ))
        } else if room_type == RoomType::Big {
            entities.push(create_minion(
                entity_id_counter,
                &rect.center(pos),
                minion_letters.pop().unwrap(),
                false,
                if !spawned_key {
                    spawned_key = true;
                    true
                } else {
                    false
                },
            ))
        } else if room_type == RoomType::Secret {
            entities.push(create_item(entity_id_counter, &rect.center(pos), Item::Key))
        }
    }

    // let mut secret_wall_positions = HashSet::<Position>::new();
    // if let Some((rect, pos, _)) = final_rooms.iter().nth(1) {
    //     for x in pos.x..pos.x + rect.width {
    //         secret_wall_positions.insert(Position { x, y: pos.y });
    //         secret_wall_positions.insert(Position {
    //             x,
    //             y: pos.y + rect.height,
    //         });
    //     }
    //     for y in pos.y..pos.y + rect.height {
    //         secret_wall_positions.insert(Position { x: pos.x, y });
    //         secret_wall_positions.insert(Position {
    //             x: pos.x + rect.width,
    //             y,
    //         });
    //     }
    // }
    // for wall_pos in secret_wall_positions {
    //     entities.push(create_wall(entity_id_counter, &wall_pos, '?'));
    // }

    for _fog_pos in fog_positions {
        // entities.push(create_fog(entity_id_counter, &fog_pos));
    }

    let all_positions: HashSet<Position> = HashSet::from_iter(
        wall_positions
            // .union(&secret_wall_positions)
            .iter()
            .cloned()
            // .map(|p| p.clone())
            .collect::<Vec<Position>>(),
    );
    let mut empty_positions = HashSet::<Position>::new();
    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let pos = Position { x, y };
            if !all_positions.contains(&pos) {
                empty_positions.insert(pos);
            }
        }
    }
    let mut all_directions = DIRECTIONS.to_vec();
    all_directions.extend(DIAGONAL_DIRECTIONS);
    for pos in all_positions.iter() {
        if all_directions
            .iter()
            .map(|(_, d)| pos + d)
            // .iter()
            .any(|p| empty_positions.contains(&p))
        {
            continue;
        }
        // TODO
        wall_positions.remove(pos);
    }
    for wall_pos in wall_positions.iter() {
        entities.push(create_wall(entity_id_counter, wall_pos, '█'));
    }

    for secrete_wall_pos in secret_wall_positions.iter() {
        entities.push(create_secret_wall(entity_id_counter, secrete_wall_pos, '█'));
    }

    (
        final_rooms
            .iter()
            .map(|f| {
                (
                    Rect {
                        width: f.0.width,
                        height: f.0.height,
                    },
                    f.1.clone(),
                    f.2.to_owned(),
                )
            })
            .collect::<Vec<(Rect, Position, RoomType)>>(),
        entities,
    )
}

fn create_wall(entity_id_counter: &mut usize, wall_pos: &Position, c: char) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some(c)),
            Component::ZIndex(Some(0)),
        ],
    )
}

fn create_secret_wall(entity_id_counter: &mut usize, wall_pos: &Position, c: char) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::SecretWall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some(c)),
            Component::ZIndex(Some(0)),
        ],
    )
}

fn create_floor(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some('.')),
            Component::ZIndex(Some(0)),
        ],
    )
}

fn create_door(entity_id_counter: &mut usize, pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Door,
            Component::Position(Some(pos.clone())),
            Component::Render(Some('$')),
            Component::ZIndex(Some(1)),
        ],
    )
}

fn create_minion(
    entity_id_counter: &mut usize,
    pos: &Position,
    c: char,
    is_boss: bool,
    spawn_key: bool,
) -> Entity {
    let mut comps = vec![
        Component::Minion(Some(is_boss)),
        Component::Position(Some(pos.clone())),
        Component::Render(Some(c)),
        Component::ZIndex(Some(1)),
    ];

    if spawn_key {
        comps.push(Component::Drop(Some(Item::Key)))
    }
    new_entity(entity_id_counter, comps)
}

pub fn create_item(entity_id_counter: &mut usize, pos: &Position, item: Item) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Position(Some(pos.clone())),
            Component::Render(Some(get_item_char(&item))),
            Component::ZIndex(Some(1)),
            Component::Item(Some(item)),
        ],
    )
}

// pub fn create_ladder(entity_id_counter: &mut usize, pos: &Position, item: Item) -> Entity {
//     new_entity(
//         entity_id_counter,
//         vec![
//             Component::Position(Some(pos.clone())),
//             Component::Render(Some(get_item_char(&item))),
//             Component::ZIndex(Some(1)),
//             Component::Ladder(),
//         ],
//     )
// }
