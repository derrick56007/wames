use std::collections::{HashMap, HashSet};

use crate::{
    components::{Component, Position},
    entity::Entity,
    state::State,
};

pub fn render(
    entities: &HashSet<usize>,
    state: &mut State,
    entities_map: &mut HashMap<usize, Box<Entity>>,
) {
    let mut buffer: Vec<char> = "."
        .repeat(state.grid_size.area() as usize)
        .chars()
        .collect();
    // println!("{:?}", entities_map);
    // println!("{:?}", entities);
    for e in entities {
        let entity = &entities_map[e];

        let position = entity
            .components
            .iter()
            .filter_map(|c| match c {
                Component::Position(a) => Some(a),
                _ => None,
            })
            .next()
            .unwrap()
            .clone()
            .unwrap();
        let idx = state.grid_size.width * position.y + position.x;
        // println!("{idx} {:?}", position);

        let render_char = entity
            .components
            .iter()
            .filter_map(|c| match c {
                Component::Render(a) => Some(a),
                _ => None,
            })
            .next()
            .unwrap()
            .clone()
            .unwrap();
        // dbg!(&position);
        // assert!(idx >= 0 && idx < buffer.len() as isize);

        buffer[idx as usize] = render_char;
    }
    for i in (0..state.grid_size.height).rev() {
        buffer.insert((i * state.grid_size.width) as usize, '\n')
    }
    print!("\x1B[2J\x1B[1;1H");

    print!("{}", &String::from_iter(buffer));
}

pub fn bresenham(pos0: &Position, pos1: &Position) -> Vec<Position> {
    let mut line = vec![];

    let dx = (pos1.x - pos0.x).abs();
    let dy = (pos1.y - pos0.y).abs();

    let sx = if pos0.x < pos1.x { 1 } else { -1 };
    let sy = if pos0.y < pos1.y { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x0 = pos0.x;
    let mut y0 = pos0.y;

    loop {
        line.push(Position { x: x0, y: y0 });

        if x0 == pos1.x && y0 == pos1.y {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err = err - dy;
            x0 = x0 + sx;
        }
        if e2 < dx {
            err = err + dx;
            y0 = y0 + sy;
        }
    }

    line
}
