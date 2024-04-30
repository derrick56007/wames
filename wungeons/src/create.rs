use colored::CustomColor;

use crate::{
    components::{get_item_char, Component, Item, Position},
    entity::{new_entity, Entity},
    event::Event,
    sight::ViewType,
};

pub fn create_wall(entity_id_counter: &mut usize, wall_pos: &Position, c: char) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some((c, None, None))),
            Component::ZIndex(Some(0)),
            Component::Solid,
        ],
    )
}

pub fn create_secret_wall(
    entity_id_counter: &mut usize,
    wall_pos: &Position,
    // c: char,
    group: usize,
) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::SecretWall(Some(group)),
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some((' ', None, None))),
            Component::ZIndex(Some(2)),
            Component::Solid,
            // Component::Viewable(Some((ViewType::SecretWall(group), vec![])))
        ],
    )
}

pub fn create_secret_wall_hint(
    entity_id_counter: &mut usize,
    wall_pos: &Position,
    // c: char,
    // group: usize,
) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::SecretWallHint,
            Component::Position(Some(wall_pos.clone())),
            // Component::Render(Some('&')),
            // Component::ZIndex(Some(4)),
            // Component::Solid,
            Component::Viewable(Some((ViewType::SecretWall, vec![]))),
        ],
    )
}

pub fn create_fog(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some((' ', None, None))),
            Component::ZIndex(Some(3)),
            Component::Fog(Some(false)),
        ],
    )
}

pub fn create_floor(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some((' ', Some(BG_COLOR), None))),
            Component::ZIndex(Some(0)),
            // Component::Fog(Some(FogState::Dark))
        ],
    )
}

pub fn create_door(entity_id_counter: &mut usize, pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Door,
            Component::Position(Some(pos.clone())),
            Component::Render(Some(('$', None, Some(WHITE)))),
            Component::ZIndex(Some(1)),
        ],
    )
}

pub fn create_minion(
    entity_id_counter: &mut usize,
    pos: &Position,
    c: char,
    looks: char,
    is_boss: bool,
    spawn_key: bool,
) -> Entity {
    let mut comps = vec![
        Component::Minion(Some((is_boss, c))),
        Component::Position(Some(pos.clone())),
        Component::Render(Some((looks, None, Some(WHITE)))),
        Component::ZIndex(Some(1)),
        Component::Viewable(Some((ViewType::Minion, vec![]))),
        Component::ViewDistance(Some(PLAYER_VIEW_DISTANCE - 1)),
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
            Component::Render(Some((get_item_char(&item), None, Some(WHITE)))),
            Component::ZIndex(Some(1)),
            Component::Item(Some(item)),
        ],
    )
}

pub const PLAYER_WALK_COOLDOWN: usize = 6;
pub const PLAYER_VIEW_DISTANCE: usize = 9;
pub const BG_COLOR: (u8, u8, u8) = (128, 128, 128);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const BLACK: (u8, u8, u8) = (0, 0, 0);

pub fn create_player(entity_id_counter: &mut usize, pos: Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Position(Some(pos)),
            Component::Render(Some(('@', Some(BG_COLOR), Some(WHITE)))),
            Component::ZIndex(Some(5)),
            Component::Player,
            Component::Cooldown(Some(PLAYER_WALK_COOLDOWN)),
            Component::AffectsFog,
            Component::ViewDistance(Some(PLAYER_VIEW_DISTANCE)),
            Component::Viewable(Some((ViewType::Player, vec![]))),
        ],
    )
}

pub fn create_dialogue(
    entity_id_counter: &mut usize,
    dialogue: Vec<(String, Option<(Option<(u8, u8, u8)>, Option<(u8, u8, u8)>)>)>,
    options: Vec<(String, Event)>,
    pos: Position,
) -> Entity {
    let index = entity_id_counter.clone();

    new_entity(
        entity_id_counter,
        vec![
            Component::Activated(Some(false)),
            Component::Dialogue(Some((dialogue, options))),
            Component::Position(Some(pos)),
            Component::ZIndex(Some(index + 5)),
        ],
    )
}
