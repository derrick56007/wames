use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use crate::{
    components::{Component, Position},
    get_component,
    state::State,
};

const COLORS: [(&str, &str); 17] = [
    ("foreground black", "\x1b[30m"),
    ("foreground red", "\x1b[31m"),
    ("foreground green", "\x1b[32m"),
    ("foreground yellow", "\x1b[33m"),
    ("foreground blue", "\x1b[34m"),
    ("foreground magenta", "\x1b[35m"),
    ("foreground cyan", "\x1b[36m"),
    ("foreground white", "\x1b[37m"),
    ("background black", "\x1b[40m"),
    ("background red", "\x1b[41m"),
    ("background green", "\x1b[42m"),
    ("background yellow", "\x1b[43m"),
    ("background blue", "\x1b[44m"),
    ("background magenta", "\x1b[45m"),
    ("background cyan", "\x1b[46m"),
    ("background white", "\x1b[47m"),
    ("reset", "\x1b[0m"),
];

fn colorize(input: String, color: &str) -> String {
    let colors: HashMap<&str, &str> = HashMap::from_iter(COLORS);

    format!("{}{input}\x1b[0m", colors[color])
}

fn colorize_foreground_rbg(input: String, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{input}\x1b[0m")
}
fn colorize_background_rbg(input: String, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{r};{g};{b}m{input}\x1b[0m")
}

pub fn render(state: &mut State, components: &[Component]) {
    let entities = state.get_entities(components);

    let mut buffer: Vec<char> = " "
        .repeat(state.grid_size.area() as usize)
        .chars()
        .collect();

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

    for (e, _) in entities.iter() {
        let entity = &state.entities_map[e];

        let position = get_component!(entity, Component::Position).unwrap();
        let idx = state.grid_size.width * position.y + position.x;

        let render_char = get_component!(entity, Component::Render).unwrap();

        buffer[idx as usize] = render_char;
    }
    for i in (0..state.grid_size.height).rev() {
        buffer.insert((i * state.grid_size.width) as usize, '\n')
    }
    print!("{}[2J", 27 as char);

    print!("\x1B[2J\x1B[1;1H");
    // let chars: HashSet<char> = HashSet::from_iter(buffer.clone());
    let mut new_buffer = "".to_string();
    for i in buffer {
        match i {
            ' ' => {
                // new_buffer = format!(
                //     "{new_buffer}{}",
                //     &colorize_background_rbg(i.to_string(), 128, 128, 128)
                // );
                new_buffer = format!("{new_buffer}{}", &colorize(' '.to_string(), "reset"));
            }
            '█' => {
                new_buffer = format!("{new_buffer}{}", &colorize(' '.to_string(), "reset"));
                // new_buffer = format!(
                //     "{new_buffer}{}",
                //     &colorize_background_rbg(i.to_string(), 128, 128, 128)
                // );
            }
            '.' => {
                // new_buffer = format!("{new_buffer}{}", &colorize(' '.to_string(), "reset"));
                new_buffer = format!(
                    "{new_buffer}{}",
                    &colorize_background_rbg(' '.to_string(), 128, 128, 128)
                );
            }
            '\n' => {
                new_buffer = format!("{new_buffer}{}", &colorize(i.to_string(), "reset"));
            }
            _ => {
                new_buffer = format!(
                    "{new_buffer}{}",
                    &colorize_background_rbg(i.to_string(), 128, 128, 128)
                );
            }
        }
    }

    print!("{}", &new_buffer);
    let mut available_letters = state
        .available_letters
        .iter()
        .copied()
        .collect::<Vec<char>>();
    available_letters.sort();
    print!(
        "\n+[ {} ] {:?}\n-[ {} ]\n", //{:?}",
        available_letters
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        state.items,
        state
            .letters_remaining
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        // SystemTime::now(),
    );
    stdout().flush().unwrap();
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
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    line
}
