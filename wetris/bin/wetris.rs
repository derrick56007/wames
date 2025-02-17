// use message_io::node::{self, NodeEvent};
// use message_io::network::{NetEvent, Transport};
// use std::time::Duration;

// enum Signal {
//     Greet,
//     // Any other app event here.
// }

// fn main() {
//     let (handler, listener) = node::split();

//     // You can change the transport to Udp or Ws (WebSocket).
//     let (server, _) = handler.network().connect(Transport::FramedTcp, "127.0.0.1:3042").unwrap();

//     listener.for_each(move |event| match event {
//         NodeEvent::Network(net_event) => match net_event {
//             NetEvent::Connected(_endpoint, _ok) => handler.signals().send(Signal::Greet),
//             NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
//             NetEvent::Message(_endpoint, data) => {
//                 println!("Received: {}", String::from_utf8_lossy(data));
//             },
//             NetEvent::Disconnected(_endpoint) => (),
//         }
//         NodeEvent::Signal(signal) => match signal {
//             Signal::Greet => { // computed every second
//                 handler.network().send(server, "Hello server!".as_bytes());
//                 handler.signals().send_with_timer(Signal::Greet, Duration::from_secs(1));
//             }
//         }
//     });
// }

use std::{
    cmp::{max, min},
    collections::HashMap,
    thread::sleep,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Tetrimino {
    Straight(Direction), // vertical and horizontal reflection symmetry, and two-fold rotational symmetry
    Square(Direction), // vertical and horizontal reflection symmetry, and four-fold rotational symmetry
    T(Direction),      // vertical reflection symmetry only
    L(Direction),      // no symmetry
    S(Direction),      // two-fold rotational symmetry only
    Z(Direction),      // two-fold rotational symmetry only
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_right(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Tetrimino {
    pub fn rotate_right(&self) -> Self {
        match self {
            Self::Straight(direction) => Self::Straight(Direction::rotate_right(direction)),
            Self::Square(direction) => Self::Square(Direction::rotate_right(direction)),
            Self::T(direction) => Self::T(Direction::rotate_right(direction)),
            Self::L(direction) => Self::L(Direction::rotate_right(direction)),
            Self::S(direction) => Self::S(Direction::rotate_right(direction)),
            Self::Z(direction) => Self::Z(Direction::rotate_right(direction)),
        }
    }

    pub fn hit(x: isize, y: isize, pos: &(isize, isize), shape: &[[usize; 4]; 4]) -> Option<usize> {
        if x >= pos.0 && x < pos.0 + 4 && y >= pos.1 && y < pos.1 + 4 {
            let x = (x - pos.0) as usize;
            let y = (y - pos.1) as usize;

            if shape[y][x] != 0 {
                return Some(shape[y][x]);
            }
        }
        None
    }
}

const SHAPES: [(Tetrimino, [[usize; 4]; 4]); 24] = [
    // Straight
    (
        Tetrimino::Straight(Direction::Up),
        [
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
        ],
    ),
    (
        Tetrimino::Straight(Direction::Right),
        [
            [0, 0, 0, 0], //
            [1, 1, 1, 1], //
            [0, 0, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Straight(Direction::Down),
        [
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
            [0, 0, 1, 0], //
        ],
    ),
    (
        Tetrimino::Straight(Direction::Left),
        [
            [0, 0, 0, 0], //
            [1, 1, 1, 1], //
            [0, 0, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    // Square
    (
        Tetrimino::Square(Direction::Up),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Square(Direction::Right),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Square(Direction::Down),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Square(Direction::Left),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    // T
    (
        Tetrimino::T(Direction::Up),
        [
            [0, 0, 0, 0], //
            [1, 1, 1, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::T(Direction::Right),
        [
            [0, 1, 0, 0], //
            [1, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::T(Direction::Down),
        [
            [0, 1, 0, 0], //
            [1, 1, 1, 0], //
            [0, 0, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::T(Direction::Left),
        [
            [0, 1, 0, 0], //
            [0, 1, 1, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    // L
    (
        Tetrimino::L(Direction::Up),
        [
            [0, 0, 0, 0], //
            [1, 1, 1, 0], //
            [1, 0, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::L(Direction::Right),
        [
            [1, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::L(Direction::Down),
        [
            [0, 0, 0, 0], //
            [0, 0, 1, 0], //
            [1, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::L(Direction::Left),
        [
            [0, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    // S
    (
        Tetrimino::S(Direction::Up),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [1, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::S(Direction::Right),
        [
            [1, 0, 0, 0], //
            [1, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::S(Direction::Down),
        [
            [0, 0, 0, 0], //
            [0, 1, 1, 0], //
            [1, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::S(Direction::Left),
        [
            [1, 0, 0, 0], //
            [1, 1, 0, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    // Z
    (
        Tetrimino::Z(Direction::Up),
        [
            [0, 0, 0, 0], //
            [1, 1, 0, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Z(Direction::Right),
        [
            [0, 0, 1, 0], //
            [0, 1, 1, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Z(Direction::Down),
        [
            [0, 0, 0, 0], //
            [1, 1, 0, 0], //
            [0, 1, 1, 0], //
            [0, 0, 0, 0], //
        ],
    ),
    (
        Tetrimino::Z(Direction::Left),
        [
            [0, 0, 1, 0], //
            [0, 1, 1, 0], //
            [0, 1, 0, 0], //
            [0, 0, 0, 0], //
        ],
    ),
];

const MINOS: [Tetrimino; 6] = [
    Tetrimino::Straight(Direction::Up),
    Tetrimino::Square(Direction::Up),
    Tetrimino::L(Direction::Up),
    Tetrimino::T(Direction::Up),
    Tetrimino::S(Direction::Up),
    Tetrimino::Z(Direction::Up),
];

fn get_mino() -> Tetrimino {
    let mut rng = rand::thread_rng();
    MINOS[rng.gen::<usize>() % MINOS.len()].clone()
}

fn get_starting_position() -> (isize, isize) {
    (3, 0)
}

const GRID_SIZE: (usize, usize) = (10usize, 20usize);

fn main() {
    let device_state = DeviceState::new();
    print!("\x1B[2J\x1B[1;1H\n");

    let mut tetrimino = get_mino();
    let mut last: Option<Keycode> = None;

    let shapes: HashMap<Tetrimino, [[usize; 4]; 4]> = HashMap::from_iter(SHAPES);
    let mut position = get_starting_position();
    let mut grid: [[usize; GRID_SIZE.0]; GRID_SIZE.1] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let mut new_mino: Option<Tetrimino> = None;
    let mut new_pos: Option<(isize, isize)> = None;
    let mut tick = 0;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        //

        // for row in shapes[&tetrimino] {
        //     for col in row {
        //         print!("{col}");
        //     }
        //     println!("");
        // }
        let keys: Vec<Keycode> = device_state.get_keys();
        if last.is_none() {
            for key in keys.iter() {
                last = Some(*key);
                break;
            }
        }
        if let Some(key) = last {
            match key {
                Keycode::Up => {
                    new_mino = Some(tetrimino.rotate_right());
                    // break 'outer;
                }
                Keycode::Right => {
                    new_pos = Some((position.0 + 1, position.1));
                    // break 'outer;
                }
                Keycode::Left => {
                    // if position.0 > 0 {
                    new_pos = Some((position.0 - 1, position.1));
                    // break 'outer;
                    // }
                }
                Keycode::Down => {
                    // if position.0 > 0 {
                    new_pos = Some((position.0, position.1 + 1));
                    // break 'outer;
                    // }
                }
                // Keycode::Q => {
                //     new_position = tetrimino.rotate_left();
                // }
                _ => {}
            }
            last = None;
        }
        {
            let mut hit = false;
            let mut out_of_bounds = false;
            // let mut hit_bottom = false;
            // check hit with grid
            'outer: for y in max(position.1, 0)..min(position.1 + 4, GRID_SIZE.1 as isize) {
                for x in max(position.0, 0)..min(position.0 + 4, GRID_SIZE.0 as isize) {
                    if grid[y as usize][x as usize] != 0 {
                        match (&new_mino, new_pos) {
                            (None, None) => {
                                break 'outer;
                            }
                            (None, Some(pos)) => {
                                if let Some(_) = Tetrimino::hit(x, y, &pos, &shapes[&tetrimino]) {
                                    hit = true;
                                    break 'outer;
                                }
                            }
                            (Some(mino), None) => {
                                if let Some(_) = Tetrimino::hit(x, y, &position, &shapes[mino]) {
                                    hit = true;
                                    break 'outer;
                                }
                            }
                            (Some(mino), Some(pos)) => {
                                if let Some(_) = Tetrimino::hit(x, y, &pos, &shapes[mino]) {
                                    hit = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
            'outer: for y in max(position.1, 0)..min(position.1 + 4, GRID_SIZE.1 as isize) {
                match (&new_mino, new_pos) {
                    (None, None) => {}
                    (None, Some(pos)) => {
                        if Tetrimino::hit(-1, y, &pos, &shapes[&tetrimino]).is_some()
                            || Tetrimino::hit(
                                GRID_SIZE.0 as isize,
                                y ,
                                &pos,
                                &shapes[&tetrimino],
                            )
                            .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                    (Some(mino), None) => {
                        if Tetrimino::hit(-1, y, &position, &shapes[&mino]).is_some()
                            || Tetrimino::hit(
                                GRID_SIZE.0 as isize,
                                y,
                                &position,
                                &shapes[&mino],
                            )
                            .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                    (Some(mino), Some(pos)) => {
                        if Tetrimino::hit(-1, y, &pos, &shapes[&mino]).is_some()
                            || Tetrimino::hit(
                                GRID_SIZE.0 as isize,
                                y ,
                                &pos,
                                &shapes[&mino],
                            )
                            .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                }
            }
            'outer: for x in max(position.0, 0)..min(position.0 + 4, GRID_SIZE.0 as isize) {
                match (&new_mino, new_pos) {
                    (None, None) => {}
                    (None, Some(pos)) => {
                        if Tetrimino::hit(
                            x,
                            GRID_SIZE.1 as isize,
                            &pos,
                            &shapes[&tetrimino],
                        )
                        .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                    (Some(mino), None) => {
                        if Tetrimino::hit(
                            x ,
                            GRID_SIZE.1 as isize,
                            &position,
                            &shapes[&mino],
                        )
                        .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                    (Some(mino), Some(pos)) => {
                        if Tetrimino::hit(x, GRID_SIZE.1 as isize, &pos, &shapes[&mino])
                            .is_some()
                        {
                            out_of_bounds = true;
                            break 'outer;
                        }
                    }
                }
            }

            // dbg!(&hit, &out_of_bounds);
            if !hit && !out_of_bounds {
                match (&new_mino, new_pos) {
                    (None, None) => {}
                    (None, Some(pos)) => {
                        position = pos;
                    }
                    (Some(mino), None) => {
                        tetrimino = mino.clone();
                    }
                    (Some(mino), Some(pos)) => {
                        position = pos;
                        tetrimino = mino.clone();
                    }
                };
            }
        }
        // if tick % 3 == 0 {
        let mut new_pos2 = (position.0, position.1);
        if tick % 5 == 0 {
            new_pos2.1 += 1;
        }
        let mut hit = false;
        let mut hit_bottom = false;
        // check hit with grid
        'outer: for x in max(new_pos2.0, 0)..min(new_pos2.0 + 4, GRID_SIZE.0 as isize) {
            for y in max(new_pos2.1, 0)..min(new_pos2.1 + 4, GRID_SIZE.1 as isize) {
                if grid[y as usize][x as usize] != 0 {
                    if let Some(_) = Tetrimino::hit(x, y, &new_pos2, &shapes[&tetrimino]) {
                        hit = true;
                        break 'outer;
                    }
                }
            }
            if Tetrimino::hit(x, GRID_SIZE.1 as isize, &new_pos2, &shapes[&tetrimino]).is_some() {
                hit_bottom = true;
                break 'outer;
            }
        }

        // dbg!(&hit, &hit_bottom);
        if !hit && !hit_bottom {
            position = new_pos2;
        } else {
            // add to grid
            for y in 0..4 {
                for x in 0..4 {
                    if shapes[&tetrimino][y as usize][x as usize] != 0 {
                        grid[(y + position.1) as usize][(x + position.0) as usize] =
                            shapes[&tetrimino][y as usize][x as usize];
                    }
                }
            }
            tetrimino = get_mino();
            position = get_starting_position();
        }
        // }
        new_pos = None;
        new_mino = None;
        tick += 1;

        for y in 0..GRID_SIZE.1 {
            print!("#");
            for x in 0..GRID_SIZE.0 {
                if let Some(hit) =
                    Tetrimino::hit(x as isize, y as isize, &position, &shapes[&tetrimino])
                {
                    print!("{hit}")
                } else if grid[y][x] == 0 {
                    print!(" ");
                } else {
                    print!("{}", grid[y][x]);
                }
            }
            print!("#");
            println!("");
        }
        println!("{}", "#".repeat(GRID_SIZE.0 + 2));

        sleep(Duration::from_millis(72));
        // let keys: Vec<Keycode> = device_state.get_keys();
        // for key in keys.iter() {
        //     last = Some(*key);
        //     break;
        // }
    }
}
