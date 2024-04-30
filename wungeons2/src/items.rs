use crate::state::State;

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub enum Item {
    Key,
    Glasses,
    MembershipCard,
    VowelVortex,
    ConsonantConundrum,
    DoubleDouble,
    DustyYam,
    SparklingToaster,
    TranquilStapler,
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

pub enum Effect {
    ShopDiscount,

    // the number of words that can be played per combat
    WordSizeChange(isize),
    DoubleLetterScore,
    DoubleWordScore,
    TripleLetterScore,
    TripleWordScore,
    BlankTile,
    RackSizeChange(isize)
    // SpeedIncrease,
}

pub fn get_item_description(
    item: &Item,
) -> Vec<(String, Option<(Option<(u8, u8, u8)>, Option<(u8, u8, u8)>)>)> {
    match item {
        Item::Key => vec![("(unlocks doors)".into(), None)],
        Item::Glasses => vec![("(+1 hand)".into(), None)],
        Item::MembershipCard => vec![("(25% discount at merchants)".into(), None)],
        Item::VowelVortex => vec![("(+1 mult for each vowel in the word)".into(), None)],
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }
}

pub fn use_item(item: &Item, _state: &mut State) {
    match item {
        Item::Key => {}
        Item::Glasses => {}
        Item::MembershipCard => {}
        Item::VowelVortex => {}
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }
}

pub fn get_item_char(item: &Item) -> String {
    match item {
        Item::Key => "⚷",
        Item::Glasses => "ው",
        Item::MembershipCard => "⌻",
        Item::VowelVortex => todo!(),
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }.to_string()
}

pub fn get_item_cost(item: &Item) -> usize {
    match item {
        Item::Key => 2,
        Item::Glasses => 4,
        Item::MembershipCard => 3,
        Item::VowelVortex => 4,
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }
}

pub fn get_item_effects(item: &Item) -> Vec<Effect> {
    match item {
        Item::Key => vec![],
        Item::Glasses => vec![Effect::WordSizeChange(1)],
        Item::MembershipCard => vec![Effect::ShopDiscount],
        Item::VowelVortex => vec![],
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }
}

pub fn get_item_use_on_pickup(item: &Item) -> bool {
    match item {
        Item::Key => false,
        Item::Glasses => false,
        Item::MembershipCard => false,
        Item::VowelVortex => true,
        Item::ConsonantConundrum => todo!(),
        Item::DoubleDouble => todo!(),
        Item::DustyYam => todo!(),
        Item::SparklingToaster => todo!(),
        Item::TranquilStapler => todo!(),
        Item::DyingFicus => todo!(),
        Item::MissingCrab => todo!(),
        Item::BreakfastCoffee => todo!(),
        Item::SpicyPillow => todo!(),
        Item::SuspiciousSausage => todo!(),
        Item::ElectricChair => todo!(),
        Item::CosmicTeapot => todo!(),
        Item::GlowingBrick => todo!(),
        Item::SizzlingRug => todo!(),
        Item::FuzzyKeyboard => todo!(),
        Item::BathroomSteak => todo!(),
    }
}
