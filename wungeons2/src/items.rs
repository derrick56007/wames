use crate::{
    effects::{get_all_modifiers_from_effects, AllModifiers, Effect},
    state::State,
};
use rand::{rngs::ThreadRng, Rng};

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Item {
    Key,
    Glasses,
    MembershipCard,
    VowelVortex,
    ConsonantConundrum,
    DoubleDouble,
    DustyYam,
    AspiringAbacus,
    TranquilScissors,
    DyingFicus,
    MissingCrab,
    BreakfastCoffee,
    SpicyPillow,
    SuspiciousSausage,
    ElectricChair,
    CosmicTeapot,
    GlowingBrick,
    SizzlingRug,
    FuzzyKeyboard,
    BathroomSteak,
}

// The "Consonant Conundrum" - 5 points per consonant
// The "Vowel Vortex" - 5 points per vowel
// The "Double-Double" - 10 points if you use a double consonant and double vowel in the same word
// The "Triple Threat" - 15 points if you use a triple consonant or triple vowel in a word
// The "Letter Lover's Delight" - 20 points if you use all seven letters on your rack in one turn
// The "Scrabble Savant" - 25 points for using a word that's seven letters or more with no duplicate letters
// The "Word Wizardry" - 30 points for playing a word that includes a Q, Z, X, or J without using a blank tile
// The "Phonetic Fortune" - 35 points if you play a word that includes at least one silent letter (e.g., "knight")
// The "Anagram Artiste" - 40 points if you rearrange all the tiles on your rack to form a new word
// The "Lexical Legend" - 50 points for playing a word that's worth 50 points or more before any bonus points are applied

// #[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
// pub enum Effect {
//     ShopDiscount,

//     // the number of words that can be played per combat
//     WordSizeChange(isize),
//     DoubleLetterScore,
//     DoubleWordScore,
//     TripleLetterScore,
//     TripleWordScore,
//     BlankTile,
//     RackSizeChange(isize)
//     // SpeedIncrease,
// }

pub fn get_random_item(rng: &mut ThreadRng) -> Item {
    let items_list = [
        Item::Key,
        Item::Glasses,
        Item::MembershipCard,
        Item::VowelVortex,
        Item::ConsonantConundrum,
        Item::DoubleDouble,
        Item::DustyYam,
        Item::AspiringAbacus,
        Item::TranquilScissors,
        Item::DyingFicus,
        Item::MissingCrab,
        Item::BreakfastCoffee,
        Item::SpicyPillow,
        Item::SuspiciousSausage,
        Item::ElectricChair,
        Item::CosmicTeapot,
        Item::GlowingBrick,
        Item::SizzlingRug,
        Item::FuzzyKeyboard,
        Item::BathroomSteak,
    ];
    items_list[rng.gen::<usize>() % items_list.len()].clone()
}

pub fn get_item_description(
    item: &Item,
) -> Vec<(String, Option<(Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>)>)> {
    match item {
        Item::Key => vec![("(does something?)".into(), None)],
        Item::Glasses => vec![("(+1 hand)".into(), None)],
        Item::MembershipCard => vec![("(25% discount at merchants)".into(), None)],
        Item::VowelVortex => vec![("(+1 mult for each vowel in the word)".into(), None)],
        Item::ConsonantConundrum => vec![("(needs description)".into(), None)],
        Item::DoubleDouble => vec![("(needs description)".into(), None)],
        Item::DustyYam => vec![("(needs description)".into(), None)],
        Item::AspiringAbacus => vec![("(needs description)".into(), None)],
        Item::TranquilScissors => vec![("(needs description)".into(), None)],
        Item::DyingFicus => vec![("(needs description)".into(), None)],
        Item::MissingCrab => vec![("(needs description)".into(), None)],
        Item::BreakfastCoffee => vec![("(needs description)".into(), None)],
        Item::SpicyPillow => vec![("(needs description)".into(), None)],
        Item::SuspiciousSausage => vec![("(needs description)".into(), None)],
        Item::ElectricChair => vec![("(needs description)".into(), None)],
        Item::CosmicTeapot => vec![("(needs description)".into(), None)],
        Item::GlowingBrick => vec![("(needs description)".into(), None)],
        Item::SizzlingRug => vec![("(needs description)".into(), None)],
        Item::FuzzyKeyboard => vec![("(needs description)".into(), None)],
        Item::BathroomSteak => vec![("(needs description)".into(), None)],
    }
}

// pub fn use_item(item: &Item, _state: &mut State) -> AllModifiers {
//     let effects: Vec<Effect> = get_item_effects(item);

//     get_all_modifiers_from_effects(effects)
// }

pub fn get_item_char(item: &Item) -> String {
    match item {
        Item::Key => "ꄗ",
        Item::Glasses => "👓",
        Item::MembershipCard => "💳",
        Item::VowelVortex => "🌀",
        Item::ConsonantConundrum => "✨",
        Item::DoubleDouble => "👥",
        Item::DustyYam => "🍠",
        Item::AspiringAbacus => "🎚",
        Item::TranquilScissors => "🪓",
        Item::DyingFicus => "🌱",
        Item::MissingCrab => "🦀",
        Item::BreakfastCoffee => "🍵",
        Item::SpicyPillow => "🐑",
        Item::SuspiciousSausage => "🌭",
        Item::ElectricChair => "💺",
        Item::CosmicTeapot => "🫖",
        Item::GlowingBrick => "🧀",
        Item::SizzlingRug => "🫓",
        Item::FuzzyKeyboard => "🎹",
        Item::BathroomSteak => "🥩",
    }
    .to_string()
}

pub fn get_item_cost(item: &Item) -> usize {
    match item {
        Item::Key => 2,
        Item::Glasses => 4,
        Item::MembershipCard => 3,
        Item::VowelVortex => 4,
        Item::ConsonantConundrum => 2,
        Item::DoubleDouble => 3,
        Item::DustyYam => 4,
        Item::AspiringAbacus => 3,
        Item::TranquilScissors => 2,
        Item::DyingFicus => 1,
        Item::MissingCrab => 4,
        Item::BreakfastCoffee => 2,
        Item::SpicyPillow => 3,
        Item::SuspiciousSausage => 3,
        Item::ElectricChair => 4,
        Item::CosmicTeapot => 2,
        Item::GlowingBrick => 4,
        Item::SizzlingRug => 3,
        Item::FuzzyKeyboard => 4,
        Item::BathroomSteak => 3,
    }
}

pub fn get_item_effects(item: &Item) -> Vec<Effect> {
    match item {
        Item::Key => vec![],
        Item::Glasses => vec![Effect::WordSizeChange(1)],
        Item::MembershipCard => vec![Effect::ShopDiscount],
        Item::VowelVortex => vec![Effect::VowelMultMultIncrease {
            mult: 1,
            curse: None,
        }],
        Item::ConsonantConundrum => vec![Effect::ConsonantMultMultIncrease {
            mult: 1,
            curse: None,
        }],
        Item::DoubleDouble => vec![Effect::ConsonantMultIncrease(2)],
        Item::DustyYam => vec![Effect::ConsonantMultIncrease(1)],
        Item::AspiringAbacus => vec![Effect::ConsonantMultIncrease(1)],
        Item::TranquilScissors => vec![],
        Item::DyingFicus => vec![],
        Item::MissingCrab => vec![],
        Item::BreakfastCoffee => vec![],
        Item::SpicyPillow => vec![],
        Item::SuspiciousSausage => vec![],
        Item::ElectricChair => vec![],
        Item::CosmicTeapot => vec![],
        Item::GlowingBrick => vec![],
        Item::SizzlingRug => vec![],
        Item::FuzzyKeyboard => vec![],
        Item::BathroomSteak => vec![],
    }
}

pub fn use_and_remove_item_on_pickup(item: &Item) -> bool {
    match item {
        Item::Key => false,
        Item::Glasses => false,
        Item::MembershipCard => false,
        Item::VowelVortex => false,
        Item::ConsonantConundrum => false,
        Item::DoubleDouble => false,
        Item::DustyYam => false,
        Item::AspiringAbacus => false,
        Item::TranquilScissors => false,
        Item::DyingFicus => false,
        Item::MissingCrab => false,
        Item::BreakfastCoffee => false,
        Item::SpicyPillow => false,
        Item::SuspiciousSausage => false,
        Item::ElectricChair => false,
        Item::CosmicTeapot => false,
        Item::GlowingBrick => false,
        Item::SizzlingRug => false,
        Item::FuzzyKeyboard => false,
        Item::BathroomSteak => false,
    }
}
