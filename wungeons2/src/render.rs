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
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};
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
    let entities = state.get_entities(components);
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
        let entity = &e;

        let position = get_component!(state.entities_map[entity], Component::Position).unwrap();

        if let Some((render_char, fg_color)) =
            get_component!(state.entities_map[entity], Component::Render)
        {
            if state.entities_map[entity].contains_component(&Component::BackgroundColor(None)) {
                if let Some(bg_color) =
                    get_component!(state.entities_map[entity], Component::BackgroundColor)
                {
                    buffers.push((
                        create_buffer('█'.to_string(), font_system),
                        position,
                        bg_color,
                    ));
                }
            }

            buffers.push((
                create_buffer(render_char.to_string(), font_system),
                position,
                fg_color,
            ));
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
            // "┏━━━━TILES━━━━┓".to_string(),
            // // "┃             ┃".to_string(),
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
