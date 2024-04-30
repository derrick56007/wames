use std::{
    collections::{HashMap, HashSet},
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

// pub fn colorize(input: String, color: &str) -> String {
//     let colors: HashMap<&str, &str> = HashMap::from_iter(COLORS);

//     format!("{}{input}\x1b[0m", colors[color])
// }

// pub fn colorize_c(input: char, color: &str) -> String {
//     let colors: HashMap<&str, &str> = HashMap::from_iter(COLORS);

//     format!("{}{input}\x1b[0m", colors[color])
// }

// fn colorize_foreground_rbg(input: String, r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[38;2;{r};{g};{b}m{input}\x1b[0m")
// }
// fn colorize_background_rbg(input: String, r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[48;2;{r};{g};{b}m{input}\x1b[0m")
// }
use colored::*;

pub fn render(state: &mut State, components: &[Component]) {
    let entities = state.get_entities(components);

    let mut buffer: Vec<String> = " "
        .repeat(state.grid_size.area() as usize)
        .chars()
        .map(|c|c.to_string())
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
    // entities.reverse();
    let mut visited_positions = HashSet::<Position>::new();

    for (e, _) in entities.iter() {
        let entity = &e;

        let position = get_component!(state.entities_map[entity], Component::Position).unwrap();
        // if visited_positions.contains(&position) {
        //     continue;
        // }
        // visited_positions.insert(position.clone());
        let idx = state.grid_size.width * position.y + position.x;

        if let Some((render_char, bg_color, fg_color)) = get_component!(state.entities_map[entity], Component::Render) {
            let mut res = render_char.to_string();
            if let Some((r, g, b)) = fg_color {
                res = res.custom_color(CustomColor { r, g, b }).to_string();
            }
            if let Some((r, g, b)) = bg_color {
                res = res.on_custom_color(CustomColor { r, g, b }).to_string();
            }
            if idx >=0 {
                buffer[idx as usize] = res;
            }
        }

    }
    for i in (0..state.grid_size.height).rev() {
        buffer.insert((i * state.grid_size.width) as usize, "\n".to_string());
    }
    print!("{}[2J", 27 as char);

    print!("\x1B[2J\x1B[1;1H");
    // let chars: HashSet<char> = HashSet::from_iter(buffer.clone());
    // let mut new_buffer = "".to_string();
    // for i in buffer {
    //     match i {
    //         // '@' => {
    //         //     new_buffer = format!("{new_buffer}{}", &colorize_background_rbg(colorize(i.to_string(), "foreground green"), 128, 128, 128));
    //         // }
    //         ' ' => {
    //             // new_buffer = format!(
    //             //     "{new_buffer}{}",
    //             //     &colorize_background_rbg(i.to_string(), 128, 128, 128)
    //             // );
    //             new_buffer = format!("{new_buffer}{}", ' ');
    //         }
    //         '█' => {
    //             new_buffer = format!("{new_buffer}{}", ' ');
    //             // new_buffer = format!(
    //             //     "{new_buffer}{}",
    //             //     &colorize_background_rbg(i.to_string(), 128, 128, 128)
    //             // );
    //         }
    //         '░' => {
    //             new_buffer = format!(
    //                 "{new_buffer}{}",
    //                i.to_string()
    //             );
    //         }
    //         '.' => {
    //             // new_buffer = format!("{new_buffer}{}", &colorize(' '.to_string(), "reset"));
    //             new_buffer = format!(
    //                 "{new_buffer}{}",
    //                ' '.to_string().on_custom_color(bg_color)
    //             );
    //         }
    //         '\n' => {
    //             new_buffer = format!("{new_buffer}{}", i.to_string());
    //         }
    //         _ => {
    //             if i.is_ascii_alphanumeric() || i == '!' || i == '?' {
    //                 new_buffer = format!("{new_buffer}{}", i.to_string());
    //             } else {
    //                 new_buffer = format!(
    //                     "{new_buffer}{}",
    //                     i.to_string().on_custom_color(bg_color)
    //                 );
    //             }
    //         }
    //     }
    // }

    // print!("{}", &new_buffer);
    print!("{}", &buffer.join(""));
    let mut available_letters = state
        .available_letters
        .iter()
        .copied()
        .collect::<Vec<char>>();
    available_letters.sort();
    // const LETTERS: &str = "Q W E R T Y U I O P\n A S D F G H J K L\n  Z X C V B N M";

    
    print!(
        "\n+[ {} ]\n-[ {} ]\n{}g {:?}\n{}", //{:?}",
        available_letters
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        state
            .letters_remaining
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        state.gold, // SystemTime::now(),
        state.items,
        state.dialogue_input
    );
    // println!("");
    // for c in LETTERS.chars() {
    //     print!(
    //         "{}",
    //         colorize(
    //             c.into(),
    //             if c == ' ' || state.letters_remaining.contains(&c)
                    
                    
    //             {
    //                 "reset"
    //             } else {
    //                 "background white"
    //             },
    //         )
    //     )
    // }
    // println!("");

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
