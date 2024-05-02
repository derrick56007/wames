use std::ops;

// use colored::CustomColor;
// use device_query::Keycode;

use winit::keyboard::KeyCode;

use crate::{event::Event, items::Item, sight::ViewType};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub const ZERO: Position = Position {x: 0, y: 0};
}

pub const DIRECTIONS: [(KeyCode , Position); 4] = [
    (KeyCode::ArrowUp, Position { x: 0, y: -1 }),
    (KeyCode::ArrowDown, Position { x: 0, y: 1 }),
    (KeyCode::ArrowLeft, Position { x: -1, y: 0 }),
    (KeyCode::ArrowRight, Position { x: 1, y: 0 }),
];

pub const DIAGONAL_DIRECTIONS: [(KeyCode, Position); 4] = [
    (KeyCode::ArrowUp, Position { x: -1, y: -1 }),
    (KeyCode::ArrowDown, Position { x: 1, y: 1 }),
    (KeyCode::ArrowLeft, Position { x: -1, y: 1 }),
    (KeyCode::ArrowRight, Position { x: 1, y: -1 }),
];

impl ops::Add<&Position> for Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<&Position> for &Position {
    type Output = Position;

    fn add(self, rhs: &Position) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// impl PartialEq for Position {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

#[derive(Clone)]
pub struct Rect {
    pub width: isize,
    pub height: isize,
}

impl Rect {
    pub fn area(&self) -> isize {
        self.width * self.height
    }

    pub fn center(&self, pos: &Position) -> Position {
        Position {
            x: pos.x + self.width / 2,
            y: pos.y + self.height / 2,
        }
    }
}

pub fn line_rect(x1: f64, y1: f64, x2: f64, y2: f64, rx: f64, ry: f64, rw: f64, rh: f64) -> bool {
    // check if the line has hit any of the rectangle's sides
    // uses the Line/Line function below
    let left = line_line(x1, y1, x2, y2, rx, ry, rx, ry + rh);
    let right = line_line(x1, y1, x2, y2, rx + rw, ry, rx + rw, ry + rh);
    let top = line_line(x1, y1, x2, y2, rx, ry, rx + rw, ry);
    let bottom = line_line(x1, y1, x2, y2, rx, ry + rh, rx + rw, ry + rh);

    // if ANY of the above are true, the line
    // has hit the rectangle
    left || right || top || bottom
}

// LINE/LINE
fn line_line(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, x4: f64, y4: f64) -> bool {
    // calculate the direction of the lines
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    // if uA and uB are between 0-1, lines are colliding
    if (0.0..=1.0).contains(&u_a) && (0.0..=1.0).contains(&u_b) {
        // optionally, draw a circle where the lines meet
        // let intersectionX = x1 + (uA * (x2 - x1));
        // let intersectionY = y1 + (uA * (y2 - y1));

        return true;
    }
    false
}
pub fn intersects(pos1: &Position, rect1: &Rect, pos2: &Position, rect2: &Rect) -> bool {
    let r1 = pos1;
    let r2 = pos2;
    let r1_width = rect1.width;
    let r2_width = rect2.width;
    let r1_height = rect1.height;
    let r2_height = rect2.height;

    r1.x <= (r2.x + r2_width)
        && (r1.x + r1_width) >= r2.x
        && r1.y <= (r2.y + r2_height)
        && (r1.y + r1_height) >= r2.y
}

pub fn contains_point(pos1: &Position, rect1: &Rect, pos2: &Position) -> bool {
    pos2.x >= pos1.x
        && pos2.x <= pos1.x + rect1.width
        && pos2.y >= pos1.y
        && pos2.y <= pos1.y + rect1.height
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Component {
    Minion(Option<bool>),
    Wall,
    SecretWall(Option<usize>),
    Room,
    Door,
    Position(Option<Position>),
    Render(Option<(String, (u8, u8, u8, u8))>),
    BackgroundColor(Option<(u8, u8, u8, u8)>),
    ZIndex(Option<isize>),
    Player,
    Drop(Option<Item>),
    Item(Option<Item>),
    Fog(Option<bool>),
    Solid,
    Dialogue(Option<(Vec<(String, Option<(Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>)>)>, Vec<(String, Event)>)>),
    DialogueChar,
    Activated(Option<bool>),
    Cooldown(Option<usize>),
    StepCount(Option<usize>),
    AffectsFog,
    ViewDistance(Option<usize>),
    Viewable(Option<(ViewType, Vec<usize>)>),
    SecretWallHint,
    Paywall(Option<usize>),
    Mystery,
    Hidden(Option<bool>)
}

// #[derive(Eq, Partia: components::ComponentlEq, Hash, Clone, Debug)]
// pub enum FogState {
//     Dark(bool),
//     Lit,
// }


pub fn get_default_component(c: &Component) -> Component {
    match c {
        Component::Position(_) => Component::Position(None),
        Component::Wall => Component::Wall,
        Component::Render(_) => Component::Render(None),
        Component::ZIndex(_) => Component::ZIndex(None),
        Component::Room => Component::Room,
        Component::Door => Component::Door,
        Component::Player => Component::Player,
        Component::Minion(_) => Component::Minion(None),
        Component::Drop(_) => Component::Drop(None),
        Component::Item(_) => Component::Item(None),
        Component::SecretWall(_) => Component::SecretWall(None),
        Component::Fog(_) => Component::Fog(None),
        Component::Solid => Component::Solid,
        Component::Dialogue(_) => Component::Dialogue(None),
        Component::DialogueChar => Component::DialogueChar,
        Component::Activated(_) => Component::Activated(None),
        Component::Cooldown(_) => Component::Cooldown(None),
        Component::StepCount(_) => Component::StepCount(None),
        Component::ViewDistance(_) => Component::ViewDistance(None),
        Component::Viewable(_) => Component::Viewable(None),
        Component::SecretWallHint => Component::SecretWallHint,
        Component::AffectsFog => Component::AffectsFog,
        Component::Paywall(_) => Component::Paywall(None),
        Component::BackgroundColor(_) => Component::BackgroundColor(None),
        Component::Mystery => Component::Mystery,
        Component::Hidden(_) => Component::Hidden(None),
    }
}
