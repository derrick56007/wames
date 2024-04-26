use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::Range,
};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{contains_point, intersects, line_rect, Component, Position, Rect},
    entity::{new_entity, Entity},
    render::bresenham,
};

const BIG_ROOM_WIDTH_RANGE: Range<usize> = 20..25;
const BIG_ROOM_HEIGHT_RANGE: Range<usize> = 10..20;

const ROOM_WIDTH_RANGE: Range<usize> = 7..10;
const ROOM_HEIGHT_RANGE: Range<usize> = 7..10;

fn generate_random_point_in_circle(rng: &mut ThreadRng, radius: isize) -> (isize, isize) {
    let random_angle = rng.gen::<f64>() * std::f64::consts::PI * 2.0;
    let random_radius = (radius as f64) * random_angle.sqrt();
    let x = random_radius * random_angle.cos();
    let y = random_radius * random_angle.sin();
    (x as isize, y as isize)
}

fn find_nearest_free_space(
    rooms: &Vec<(Rect, Position)>,
    room_index: usize,
    // grid_size: &Rect,
    rng: &mut ThreadRng,
) -> Option<Position> {
    let (room, pos) = &rooms[room_index];

    for (i, (other_room, other_pos)) in rooms.iter().enumerate() {
        if i != room_index {
            if intersects(pos, room, other_pos, other_room) {
                return Some(Position {
                    x: pos.x + rng.gen_range(-1..2),
                    y: pos.y + rng.gen_range(-1..2),
                });
            }
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
    rooms: &mut Vec<(Rect, Position)>,
    grid_size: &Rect,
    rng: &mut ThreadRng,
    start: usize,
) {
    for room_index in start..rooms.len() {
        'inner: loop {
            if let Some(new_pos) = find_nearest_free_space(&rooms, room_index, rng) {
                let (room_rect, _) = &rooms[room_index];
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

pub fn create_rooms(
    rng: &mut ThreadRng,
    grid_size: &Rect,
    entity_id_counter: &mut usize,
) -> Vec<Entity> {
    let mut rooms: Vec<(Rect, Position)> = vec![];
    let num_big_rooms = 3;
    let num_small_rooms = 5;
    let num_rooms = 10;
    let mut big_lines: Vec<(Position, Position)> = vec![];

    for _ in 0..num_small_rooms {
        let rect = Rect {
            width: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((rect, pos));
    }

    resolve_collisions(&mut rooms, grid_size, rng, 0);

    for _ in 0..num_big_rooms {
        let rect = Rect {
            width: rng.gen_range(BIG_ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(BIG_ROOM_HEIGHT_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((rect, pos));
    }

    resolve_collisions(&mut rooms, grid_size, rng, num_small_rooms);

    for _ in 0..num_rooms {
        let rect = Rect {
            width: rng.gen_range(ROOM_WIDTH_RANGE) as isize,
            height: rng.gen_range(ROOM_HEIGHT_RANGE) as isize,
        };
        let pos = generate_rand_position(rng, grid_size, &rect);
        rooms.push((rect, pos));
    }

    // resolve room positions
    resolve_collisions(&mut rooms, grid_size, rng, num_small_rooms + num_big_rooms);

    for i in 1..num_big_rooms + num_small_rooms {
        let (last_rect, last_pos) = &rooms[i - 1];
        let (rect, pos) = &rooms[i];
        big_lines.push((last_rect.center(&last_pos), rect.center(&pos)))
    }

    let mut final_rooms = vec![];

    for (i, (rect, pos)) in rooms.iter().enumerate() {
        if i >= num_big_rooms + num_small_rooms {
            if !big_lines.iter().any(|(pos1, pos2)| {
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
            }) {
                continue;
            }
        }
        final_rooms.push((rect, pos));
    }
    let mut hallways = vec![];
    for i in 1..num_big_rooms + num_small_rooms {
        let (last_rect, last_pos) = &rooms[i - 1];
        let (rect, pos) = &rooms[i];
        let last_center = last_rect.center(&last_pos);
        let center = rect.center(&pos);
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
    }
    let mut wall_positions: HashSet<Position> = HashSet::new();

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            wall_positions.insert(Position { x, y });
        }
    }

    // remove hallways
    for (pos1, pos2) in hallways {
        for line_pos in bresenham(&pos1, &pos2) {
            wall_positions.remove(&line_pos);
        }
    }

    let secret_rooms_count = 1;
    // 'outer: for (rect, pos) in rooms.iter() {
    //     for (f_rect, f_pos) in final_rooms.iter() {
    //         if f_rect.width == rect.width
    //             && f_rect.height == rect.height
    //             && f_pos.x == pos.x
    //             && f_pos.y == pos.y
    //         {
    //             continue 'outer;
    //         }
    //     }
    //     if secret_rooms_count > 0 {
    //         final_rooms.push((rect, pos))
    //         secret_rooms_count -= 1;
    //     } else {
    //         break;
    //     }
    // }

    for (rect, pos) in final_rooms.iter() {
        for x in pos.x + 1..pos.x + rect.width {
            for y in pos.y + 1..pos.y + rect.height {
                wall_positions.remove(&Position { x, y });
            }
        }
    }

    let mut secret_wall_positions = HashSet::<Position>::new();
    for (rect, pos) in final_rooms.iter().take(secret_rooms_count) {
        for x in pos.x..pos.x + rect.width {
            secret_wall_positions.insert(Position { x, y: pos.y });
            secret_wall_positions.insert(Position {
                x,
                y: pos.y + rect.height,
            });
        }
        for y in pos.y..pos.y + rect.height {
            secret_wall_positions.insert(Position { x: pos.x, y });
            secret_wall_positions.insert(Position {
                x: pos.x + rect.width,
                y,
            });
        }
    }

    let mut entities = vec![];

    for wall_pos in wall_positions {
        entities.push(new_entity(
            entity_id_counter,
            vec![
                Component::Wall,
                Component::Position(Some(wall_pos.clone())),
                Component::Render(Some('@')),
            ],
        ));
    }

    for wall_pos in secret_wall_positions {
        entities.push(new_entity(
            entity_id_counter,
            vec![
                Component::Wall,
                Component::Position(Some(wall_pos.clone())),
                Component::Render(Some('#')),
            ],
        ));
    }

    entities
}
