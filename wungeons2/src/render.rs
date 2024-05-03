use std::iter::zip;

use crate::{
    components::{Component, Position},
    get_component,
    items::get_item_char,
    state::State,
    TILE_HEIGHT, TILE_WIDTH,
};

use crate::colors::*;

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

pub fn superscript_v2_collect(mut value: usize) -> String {
    const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let mut started = false;
    let mut power_of_ten = 1_000_000_000;
    if value == 0 {
        '⁰'.to_string()
    } else {
        (0..10)
            .filter_map(|_| {
                let digit = value / power_of_ten;
                value -= digit * power_of_ten;
                power_of_ten /= 10;
                if digit != 0 || started {
                    started = true;
                    Some(SUPERSCRIPT_DIGITS[digit as usize])
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Show {
    None,
    Tiles,
    Items,
}

// pub fn colorize(input: String, color: &str) -> String {
//     let colors: FnvHashMap<&str, &str> = FnvHashMap::from_iter(COLORS);

//     format!("{}{input}\x1b[0m", colors[color])
// }

// pub fn colorize_c(input: char, color: &str) -> String {
//     let colors: FnvHashMap<&str, &str> = FnvHashMap::from_iter(COLORS);

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
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
// use colored::Colorize;

pub fn render(
    _width: i32,
    _height: i32,
    font_system: &mut FontSystem,
    scale_factor: f64,
    state: &mut State,
    components: &[Component],
    buffers: &mut Vec<(Buffer, Position, (u8, u8, u8, u8), f32, f32, bool)>,
) {
    let entities = state.get_entities(components);
    let mut entities = entities
        .iter()
        .map(|e| {
            (
                *e,
                (
                    get_component!(state.entities_map[e], Component::ZIndex).unwrap(),
                    get_component!(state.entities_map[e], Component::Position)
                        .unwrap()
                        .y,
                ),
            )
        })
        .collect::<Vec<(usize, (isize, isize))>>();
    // entities.sort_by(|a, b| a.1.cmp(&b.1));

    // let mut renderables = vec![];

    for (e, _) in entities.iter() {
        let entity = &e;

        if state.fog_enabled
            && state.entities_map[entity].contains_component(&Component::Hidden(None))
        {
            if get_component!(state.entities_map[entity], Component::Hidden).unwrap() {
                continue;
            }
        }
        let position = get_component!(state.entities_map[entity], Component::Position).unwrap();
        let z = get_component!(state.entities_map[entity], Component::ZIndex).unwrap() as f32;

        if state.entities_map[entity].contains_component(&Component::RenderBg(None)) {
            let (render_char, bg_color) =
                get_component!(state.entities_map[entity], Component::RenderBg).unwrap();

            buffers.push((
                create_buffer(render_char, font_system),
                position,
                bg_color,
                0.0,
                (position.y + TILE_HEIGHT) as f32 + z,
                false,
            ));
        }
        if state.entities_map[entity].contains_component(&Component::RenderFg(None)) {
            let (render_char, fg_color, center) =
                get_component!(state.entities_map[entity], Component::RenderFg).unwrap();

            let offset = if state.entities_map[entity].contains_component(&Component::DialogueChar)
            {
                0.0
            } else if state.entities_map[entity].contains_component(&Component::Wall)
                || state.entities_map[entity].contains_component(&Component::SecretWall(None))
            {
                -TILE_HEIGHT as f32 / 2.0
            } else if state.entities_map[entity].contains_component(&Component::Player) {
                -TILE_HEIGHT as f32 / 4.0
            } else {
                -TILE_HEIGHT as f32 / 3.0
            };

            buffers.push((
                create_buffer(render_char.to_string(), font_system),
                position,
                fg_color,
                offset,
                (position.y + TILE_HEIGHT) as f32 - offset + z,
                center,
            ));
        }
    }

    // for (e, _) in entities.iter() {
    //     let entity = &e;

    //     if state.fog_enabled
    //         && state.entities_map[entity].contains_component(&Component::Invisible(None))
    //     {
    //         if get_component!(state.entities_map[entity], Component::Invisible).unwrap() {
    //             continue;
    //         }
    //     }
    //     let position = get_component!(state.entities_map[entity], Component::Position).unwrap();
    //     if let Some((render_char, fg_color)) =
    //         get_component!(state.entities_map[entity], Component::Render)
    //     {
    //         buffers.push((
    //             create_buffer(render_char.to_string(), font_system),
    //             position,
    //             fg_color,
    //             if state.entities_map[entity].contains_component(&Component::DialogueChar) {
    //                 0.0
    //             } else {
    //                 -TILE_HEIGHT as f32 / 2.0
    //             },
    //             position.y as f32,
    //         ));
    //     }
    // }

    for (x, c) in state.dialogue_input.chars().enumerate() {
        buffers.push((
            create_buffer(c.to_string(), font_system),
            Position {
                x: x as isize,
                y: state.grid_size.height,
            },
            WHITE,
            0.0,
            state.grid_size.height as f32,
            false,
        ));
    }

    let mut additional_post: Vec<String> = vec![
        "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┓".into(),
        format!("┃ Words       {: >13} ┃", 5),
        format!("┃ Gold        {: >13} ┃", state.gold),
        format!("┃ Rack        {: >13} ┃", state.gold),
        format!("┃ Discard     {: >13} ┃", state.gold),
        format!("┃ Floor       {: >13} ┃", state.floor),
       "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┛".into(),
        ];
    match state.show {
        Show::Tiles => {
            additional_post.push("┏━━━━━━━━━━━━━┳━━━━━━━━━━━━━┓".to_string());
            additional_post.push("┃             ┃             ┃".into());

            for (char_1, char_2) in zip("ABCDEFGHIJKLM".chars(), "NOPQRSTUVWXYZ".chars()) {
                additional_post.push(format!(
                    "┃ {: <6}× {: >3} ┃ {: <6}× {: >3} ┃",
                    format!("{char_1}⁽{}⁾",
                    superscript_v2_collect(state.tile_points[&char_1])),
                    state
                        .available_letters
                        .iter()
                        .filter(|l| **l == char_1)
                        .count(),
                        format!("{char_2}⁽{}⁾",
                    superscript_v2_collect(state.tile_points[&char_2])),
                    state
                        .available_letters
                        .iter()
                        .filter(|l| **l == char_2)
                        .count()
                ));
            additional_post.push("┃             ┃             ┃".into());

            }
            additional_post.push("┗━━━━━━━━━━━━━┻━━━━━━━━━━━━━┛".into());
        }
        Show::None => {
            additional_post.clear();
        }
        Show::Items => {
            additional_post.push("┏━━━━━ITEMS━━━━━┓".to_string());

            if !state.items.is_empty() {
                for chunk in state.items.chunks(5) {
                    let mut s = " ".to_string();
                    for c in chunk {
                        s = format!("{s} {}", get_item_char(c));
                    }
                    additional_post.push(s);
                }
            }
            additional_post.push("┗━━━━━━━━━━━━━━━┛".into());
        }
    }

    for (i, line) in additional_post.iter().enumerate() {
        buffers.push((
            create_buffer(line.to_string(), font_system),
            Position {
                x: state.grid_size.width,
                y: i as isize,
            },
            WHITE,
            0.0,
            i as f32,
            false,
        ));
    }
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
    buffer.set_size(
        font_system,
        physical_width * text.len() as f32,
        physical_height,
    );
    buffer.set_text(
        font_system,
        &text,
        Attrs::new().family(Family::Monospace),
        Shaping::Advanced,
    );

    buffer

    // buffers.push((buffer, Position{x: x as isize, y: state.grid_size.height}, WHITE));
}
