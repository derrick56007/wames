// use colored::CustomColor;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Component, Position}, entity::{new_entity, Entity}, event::Event, items::{get_item_char, get_item_cost, Item}, sight::ViewType
};

pub fn create_wall(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some(('█', BLACK))),
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
            Component::Render(Some(('█', BLACK))),
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
            Component::Render(Some(('█', BLACK))),
            Component::ZIndex(Some(5)),
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
            Component::Render(Some(('█', BG_COLOR))),
            Component::ZIndex(Some(0)),
            // Component::Fog(Some(FogState::Dark))
        ],
    )
}

pub fn create_revealed_floor(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(wall_pos.clone())),
            Component::Render(Some(('█', REVEALED_BG_COLOR))),
            Component::ZIndex(Some(1)),
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
            Component::Render(Some(('$', WHITE))),
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
        Component::Render(Some((looks, WHITE))),
        Component::ZIndex(Some(1)),
        Component::Viewable(Some((ViewType::Minion, vec![]))),
        Component::ViewDistance(Some(PLAYER_VIEW_DISTANCE - 2)),
    ];

    if spawn_key {
        comps.push(Component::Drop(Some(Item::Key)))
    }
    new_entity(entity_id_counter, comps)
}

pub fn create_item(rng: &mut ThreadRng, entity_id_counter: &mut usize, pos: &Position, item: Option<Item>, cost: Option<usize>) -> Entity {
    let items_list = [
        Item::Glasses,
        Item::Key,
    ];
    let item = if item.is_none() {
        items_list[rng.gen::<usize>() % items_list.len()].clone()
    } else {
        item.unwrap()
    };
    let cost = if cost.is_none() {
        get_item_cost(&item)
    } else {
        cost.unwrap()
    };
    
    let mut comps = vec![
        Component::Position(Some(pos.clone())),
        Component::Render(Some((get_item_char(&item), GOLD))),
        Component::ZIndex(Some(4)),
        Component::Item(Some(item)),
        Component::Paywall(Some(cost))
    ];
    // if cost > 0 {
    //     comps.push(Component::Paywall(Some(cost)));
    // }
    new_entity(
        entity_id_counter,
        comps,
    )
}

pub const PLAYER_WALK_COOLDOWN: usize = 5;
pub const PLAYER_VIEW_DISTANCE: usize = 9;
pub const BG_COLOR: (u8, u8, u8) 
= (78,54,42);
pub const REVEALED_BG_COLOR: (u8, u8, u8) 
= (61,43,31);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const BLACK: (u8, u8, u8) = (0, 0, 0);
pub const GOLD: (u8, u8, u8) = (218, 145, 1);

pub fn create_player(entity_id_counter: &mut usize, pos: Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Position(Some(pos)),
            Component::Render(Some(('⛄', BLACK))),
            // Component::BackgroundColor(Some((255, 0, 0))),
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
