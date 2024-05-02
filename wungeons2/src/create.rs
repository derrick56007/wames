// use colored::CustomColor;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    colors::*, components::{Component, Position}, effects::{get_effect_description, get_random_effect, Effect}, entity::{new_entity, Entity}, event::Event, items::{get_item_char, get_item_cost, get_random_item, Item}, sight::ViewType
};

pub fn create_wall(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Wall,
            Component::Position(Some(*wall_pos)),
            Component::Render(Some(('█'.to_string(), (50, 50, 50, 255)))),
            Component::BackgroundColor(Some((40, 40, 40, 255))),
            Component::ZIndex(Some(7)),
            Component::Invisible(Some(true)),
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
            Component::Position(Some(*wall_pos)),
            Component::Render(Some(('█'.to_string(), BLACK))),
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
            Component::Position(Some(*wall_pos)),
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
            Component::Position(Some(*wall_pos)),
            Component::Render(Some(('█'.to_string(), BLACK))),
            Component::BackgroundColor(Some(FOG_BG_COLOR)),
            Component::ZIndex(Some(10)),
            Component::Fog(Some(false)),
            Component::Invisible(Some(true))//ᒉ
        ],
    )
}

pub fn create_floor(entity_id_counter: &mut usize, wall_pos: &Position, color: (u8, u8, u8, u8), z: isize) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(*wall_pos)),
            Component::Render(Some((' '.to_string(), (0, 0, 0, 0)))),
            Component::BackgroundColor(Some(color)),
            Component::ZIndex(Some(z)),
            Component::Invisible(Some(true))//ᒉ
            // Component::Fog(Some(FogState::Dark))
        ],
    )
}

pub fn create_pedastal(entity_id_counter: &mut usize, wall_pos: &Position, color: (u8, u8, u8, u8)) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(*wall_pos)),
            Component::Render(Some(('▄'.to_string(), color))),
            Component::ZIndex(Some(1)),
            Component::Invisible(Some(true)),
            Component::BackgroundColor(Some(darken_color(color)))
            // Component::Fog(Some(FogState::Dark))
        ],
    )
}

pub fn create_plant(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(*wall_pos)),
            Component::Render(Some(('ᒉ'.to_string(), (0, 255, 0, 255)))),
            // Component::BackgroundColor(Some((0,0,255,100))),
            Component::ZIndex(Some(2)),
            Component::Invisible(Some(true))//
            // Component::Fog(Some(FogState::Dark))
        ],
    )
}

pub fn create_revealed_floor(entity_id_counter: &mut usize, wall_pos: &Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            // Component::Wall,
            Component::Position(Some(*wall_pos)),
            Component::Render(Some((' '.to_string(), (0, 0, 0, 0)))),
            Component::BackgroundColor(Some(REVEALED_BG_COLOR)),

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
            Component::Position(Some(*pos)),
            Component::Render(Some(('$'.to_string(), WHITE))),
            Component::ZIndex(Some(0)),
            Component::Invisible(Some(true))//ᒉ
        ],
    )
}

pub fn create_minion(
    entity_id_counter: &mut usize,
    pos: &Position,
    c: char,
    looks: String,
    is_boss: bool,
    spawn_key: bool,
) -> Entity {
    let mut comps = vec![
        Component::Minion(Some((is_boss, c))),
        Component::Position(Some(*pos)),
        Component::Render(Some((looks, WHITE))),
        Component::ZIndex(Some(0)),
        Component::Viewable(Some((ViewType::Minion, vec![]))),
        Component::ViewDistance(Some(PLAYER_VIEW_DISTANCE - 2)),
        Component::Invisible(Some(true))//ᒉ
    ];

    if spawn_key {
        comps.push(Component::Drop(Some(Item::Key)))
    }
    new_entity(entity_id_counter, comps)
}

pub fn create_mystery(
    rng: &mut ThreadRng,
    entity_id_counter: &mut usize,
    pos: &Position,
) -> Entity {
    let comps = vec![
        Component::Position(Some(*pos)),
        Component::Render(Some(('?'.into(), BLACK))),
        Component::ZIndex(Some(4)),
        Component::Mystery,
    ];
    // if cost > 0 {
    //     comps.push(Component::Paywall(Some(cost)));
    // }
    new_entity(entity_id_counter, comps)
}

pub fn create_item(
    rng: &mut ThreadRng,
    entity_id_counter: &mut usize,
    pos: &Position,
    item: Option<Item>,
    cost: Option<usize>,
) -> Entity {
    let item = if item.is_none() {
        get_random_item(rng)
    } else {
        item.unwrap()
    };
    let cost = if cost.is_none() {
        get_item_cost(&item)
    } else {
        cost.unwrap()
    };

    let comps = vec![
        Component::Position(Some(*pos)),
        Component::Render(Some((get_item_char(&item), GOLD))),
        Component::ZIndex(Some(4)),
        Component::Item(Some(item)),
        Component::Paywall(Some(cost)),
        Component::Invisible(Some(true))//ᒉ
    ];
    // if cost > 0 {
    //     comps.push(Component::Paywall(Some(cost)));
    // }
    new_entity(entity_id_counter, comps)
}

pub const PLAYER_WALK_COOLDOWN: usize = 5;
pub const PLAYER_VIEW_DISTANCE: usize = 9;

pub fn create_player(entity_id_counter: &mut usize, pos: Position) -> Entity {
    new_entity(
        entity_id_counter,
        vec![
            Component::Position(Some(pos)),
            Component::Render(Some(("👳🏾".to_string(), BLACK))), //👳🏾
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

pub fn create_mystery_dialogue(entity_id_counter: &mut usize, rng: &mut ThreadRng) -> Entity {
    let mut effects: Vec<Effect> = vec![];

    loop {
        if effects.len() == 3 {
            break;
        }
        let effect = get_random_effect(rng);

        if effects.contains(&effect) {
            continue;
        }

        effects.push(effect);
    }
    let options: Vec<(String, Event)> = effects
        .iter()
        .enumerate()
        .map(|(i, e)| (format!("{}", i + 1), Event::UseEffect(e.clone())))
        .collect();
    let dialogue = vec![
        ("A whisper comes from the haze..\n".to_string(), None),
        (
            "I can grant you one of three wishes..\n\n".to_string(),
            None,
        ),
        ((
            format!("1) {}\n", get_effect_description(&effects[0]).to_string()),
            None,
        )),
        ((
            format!("2) {}\n", get_effect_description(&effects[1]).to_string()),
            None,
        )),
        ((
            format!("3) {}\n", get_effect_description(&effects[2]).to_string()),
            None,
        )),
    ];
    let index = *entity_id_counter;

    new_entity(
        entity_id_counter,
        vec![
            Component::Activated(Some(false)),
            Component::Dialogue(Some((dialogue, options))),
            Component::Position(Some(Position::ZERO)),
            Component::ZIndex(Some((index as isize) + 5)),
        ],
    )
}

pub fn create_dialogue(
    entity_id_counter: &mut usize,
    dialogue: Vec<(
        String,
        Option<(Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>)>,
    )>,
    options: Vec<(String, Event)>,
    pos: Position,
) -> Entity {
    let index = *entity_id_counter;

    new_entity(
        entity_id_counter,
        vec![
            Component::Activated(Some(false)),
            Component::Dialogue(Some((dialogue, options))),
            Component::Position(Some(pos)),
            Component::ZIndex(Some((index as isize) + 5)),
        ],
    )
}
