

use crate::{
    components::{Component, Position},
    create::WHITE,
    get_component,
    state::State,
    TILE_HEIGHT, TILE_WIDTH,
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

// use colored::CustomColor;
// fn colorize_foreground_rbg(input: String, r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[38;2;{r};{g};{b}m{input}\x1b[0m")
// }
// fn colorize_background_rbg(input: String, r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[48;2;{r};{g};{b}m{input}\x1b[0m")
// }
// use colored::*;
use glyphon::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping,
};
// use colored::Colorize;

pub fn render(
    _width: i32,
    _height: i32,
    font_system: &mut FontSystem,
    scale_factor: f64,
    state: &mut State,
    components: &[Component],
    buffers: &mut Vec<(Buffer, Position, (u8, u8, u8))>,
) {
    // let mut text_areas: Vec<TextArea> = vec![];
    let entities = state.get_entities(components);

    // let mut buffer: Vec<String> = " "
    //     .repeat(state.grid_size.area() as usize)
    //     .chars()
    //     .map(|c| c.to_string())
    //     .collect();

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
    // let mut visited_positions = HashSet::<Position>::new();

    let _physical_width = (TILE_WIDTH as f64 * scale_factor) as f32;
    let _physical_height = (TILE_HEIGHT as f64 * scale_factor) as f32;
    for (e, _) in entities.iter() {
        let entity = &e;

        let position = get_component!(state.entities_map[entity], Component::Position).unwrap();
        // if visited_positions.contains(&position) {
        //     continue;
        // }
        // visited_positions.insert(position.clone());
        // let idx = state.grid_size.width * position.y + position.x;

        if let Some((render_char, fg_color)) =
            get_component!(state.entities_map[entity], Component::Render)
        {
            if state.entities_map[entity].contains_component(&Component::BackgroundColor(None)) {
                if let Some(bg_color) =
                    get_component!(state.entities_map[entity], Component::BackgroundColor)
                {
                    // let mut buffer = Buffer::new(
                    //     &mut font_system,
                    //     Metrics::new(TILE_HEIGHT as f32, TILE_HEIGHT as f32),
                    // );

                    // buffer.set_size(&mut font_system, physical_width, physical_height);
                    // buffer.set_text(
                    //     &mut font_system,
                    //     &'█'.to_string(),
                    //     Attrs::new().family(Family::Monospace),
                    //     Shaping::Advanced,
                    // );

                    buffers.push((
                        create_buffer('█'.to_string(), font_system),
                        position,
                        bg_color,
                    ));
                }
            }
            // let mut buffer = Buffer::new(
            //     &mut font_system,
            //     Metrics::new(TILE_HEIGHT as f32, TILE_HEIGHT as f32),
            // );

            // buffer.set_size(&mut font_system, physical_width, physical_height);
            // buffer.set_text(
            //     &mut font_system,
            //     &render_char.to_string(),
            //     Attrs::new().family(Family::Monospace),
            //     Shaping::Advanced,
            // );

            buffers.push((
                create_buffer(render_char.to_string(), font_system),
                position,
                fg_color,
            ));
            // }
            //     TextArea {
            //     buffer: &buffer,
            //     left: 10.0,
            //     top: 10.0,
            //     scale: 1.0,
            //     bounds: TextBounds {
            //         left: 0,
            //         top: 0,
            //         right: 600,
            //         bottom: 160,
            //     },
            //     default_color: Color::rgb(255, 255, 255),
            // }
        }
    }

    for (x, c) in state.dialogue_input.chars().enumerate() {
        buffers.push((
            create_buffer(c.to_string(), font_system),
            Position {
                x: x as isize,
                y: state.grid_size.height,
            },
            WHITE,
        ));
    }

    if state.show_deck {
        let additional_post: Vec<String> = vec![
            "┏━━━━━━━━━━━━━┓".into(),
            format!("┃ Words: {: >4} ┃", 5),
            format!("┃ Gold : {: >4} ┃", state.gold),
            format!("┃ Floor: {: >4} ┃", state.floor),
            "┗━━━━━━━━━━━━━┛".into(),
            "┏━━━━TILES━━━━┓".to_string(),
            // "┃             ┃".to_string(),
        ];
        for (i, line) in additional_post.iter().enumerate() {
            buffers.push((
                create_buffer(line.to_string(), font_system),
                Position {
                    x: state.grid_size.width,
                    y: i as isize,
                },
                WHITE,
            ));
        }
    }

    // let left_side_width = 20;

    // let mut additional_post: Vec<String> = vec![
    //     "┏━━━━━━━━━━━━━┓".into(),
    //     format!("┃ Words: {: >4} ┃", 5),
    //     format!("┃ Gold : {: >4} ┃", state.gold),
    //     format!("┃ Floor: {: >4} ┃", state.floor),
    //     "┗━━━━━━━━━━━━━┛".into(),
    //     "┏━━━━TILES━━━━┓".to_string(),
    //     // "┃             ┃".to_string(),
    // ];
    // let mut freq = HashMap::<char, usize>::new();
    // for l in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
    //     if !freq.contains_key(&l) {
    //         freq.insert(l, 0);
    //     }
    //     if !state.available_letters.contains(&l) {
    //         continue;
    //     }
    //     freq.insert(l, freq[&l] + 1);
    // }
    // for (l1, l2) in zip("ABCDEFGHIJKLM".chars(), "NOPQRSTUVWXYZ".chars()) {
    //     // if !freq.contains_key(&l) {
    //     //     continue;
    //     // }
    //     additional_post.push(format!("┃ {} x{} ┃ {} x{} ┃", l1, freq[&l1], l2, freq[&l2]))
    // }
    // additional_post.push("┗━━━━━━━━━━━━━┛".into());

    // if !state.show_deck {
    //     additional_post.clear();
    // }

    // for i in (0..state.grid_size.height as usize).rev() {
    //     // let p = if i < additional_pre.len() {
    //     //     if i == 0 {
    //     //         "".into()
    //     //     } else {

    //     //         format!(
    //     //             "{}{}",
    //     //             additional_pre[i],
    //     //             " ".repeat(left_side_width - additional_pre[i].len())
    //     //         )
    //     //     }
    //     // } else {
    //     //     " ".repeat(left_side_width)
    //     // };

    //     // buffer.insert(
    //     //     (i) * state.grid_size.width as usize,
    //     //     p.to_string(),
    //     // );

    //     buffer.insert(i * state.grid_size.width as usize, "\n".to_string());

    //     if i < additional_post.len() {
    //         buffer.insert(
    //             (i + 2) * state.grid_size.width as usize,
    //             additional_post[i].to_string(),
    //         );
    //     }
    // }
    // print!("{}[2J", 27 as char);

    // print!("\x1B[2J\x1B[1;1H");
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
    // print!("{}", &buffer.join(""));
    // let mut available_letters = state
    //     .available_letters
    //     .iter()
    //     .copied()
    //     .collect::<Vec<char>>();
    // available_letters.sort();
    // const LETTERS: &str = "Q W E R T Y U I O P\n A S D F G H J K L\n  Z X C V B N M";

    // print!(
    //     "\n+[ {} ]\n-[ {} ]\n{}g {:?}\n{}", //{:?}",
    //     available_letters
    //         .iter()
    //         .map(|c| c.to_string())
    //         .collect::<Vec<String>>()
    //         .join(", "),
    //     state
    //         .letters_remaining
    //         .iter()
    //         .map(|c| c.to_string())
    //         .collect::<Vec<String>>()
    //         .join(", "),
    //     state.gold, // SystemTime::now(),
    //     state.items,
    //     state.dialogue_input
    // );

    // print!("\n{}", state.dialogue_input);
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

    // stdout().flush().unwrap();
    // text_areas
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

pub fn create_buffer(text: String, mut font_system: &mut FontSystem) -> Buffer {
    let mut buffer = Buffer::new(
        font_system,
        Metrics::new(TILE_HEIGHT as f32, TILE_HEIGHT as f32),
    );

    let physical_width = (TILE_WIDTH as f64) as f32;
    let physical_height = (TILE_HEIGHT as f64) as f32;
    buffer.set_size(font_system, physical_width * text.len() as f32, physical_height);
    buffer.set_text(
        font_system,
        &text,
        Attrs::new().family(Family::Monospace),
        Shaping::Advanced,
    );

    buffer

    // buffers.push((buffer, Position{x: x as isize, y: state.grid_size.height}, WHITE));
}
