use std::ops;

use rand::{rngs::ThreadRng, Rng};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Effect {
    ShopDiscount,
    // the number of words that can be played per combat
    WordSizeChange(isize),
    // DoubleLetterScore,
    // DoubleWordScore,
    // TripleLetterScore,
    // TripleWordScore,
    // BlankTile,
    GainGold(usize),
    VowelMultIncrease(usize),
    VowelMultMultIncrease {
        mult: usize,
        curse: Option<Box<Effect>>,
    },
    ConsonantMultIncrease(usize),
    ConsonantMultMultIncrease {
        mult: usize,
        curse: Option<Box<Effect>>,
    },
    RackSizeChange(isize), // SpeedIncrease,
}

#[derive(Default)]
pub struct AllModifiers {
    pub gold: usize,
    pub shop_discount: usize,
    pub word_size_change: isize,
    pub mults: Vec<(LetterType, MultType, usize)>,
    pub rack_size_change: isize,
}

impl AllModifiers {
    pub fn add(&mut self, rhs: &mut AllModifiers) {
        self.gold += rhs.gold;
        self.shop_discount += rhs.shop_discount;
        self.word_size_change += rhs.word_size_change;
        self.mults.append(&mut rhs.mults)
    }
}

pub enum LetterType {
    Consonant,
    Vowel,
}

pub enum MultType {
    Increase,
    Mult,
}

pub fn get_all_modifiers_from_effects(mods: &mut AllModifiers, effects: Vec<Effect>) {
    'outer: for e in effects {
        match e {
            Effect::ShopDiscount => {
                mods.shop_discount += 1;
            }
            Effect::WordSizeChange(change) => {
                mods.word_size_change += change;
            }
            Effect::GainGold(change) => {
                mods.gold += change;
            }
            Effect::VowelMultIncrease(mult_increase) => {
                mods.mults
                    .push((LetterType::Vowel, MultType::Increase, mult_increase))
            }
            Effect::VowelMultMultIncrease { mult, curse } => {
                mods.mults.push((LetterType::Vowel, MultType::Mult, mult));

                if curse.is_none() {
                    continue 'outer;
                }
                get_all_modifiers_from_effects(mods, vec![*curse.unwrap()]);
            }
            Effect::ConsonantMultIncrease(mult_increase) => {
                mods.mults
                    .push((LetterType::Consonant, MultType::Increase, mult_increase))
            }
            Effect::ConsonantMultMultIncrease { mult, curse } => {
                mods.mults.push((LetterType::Consonant, MultType::Mult, mult));

                if curse.is_none() {
                    continue 'outer;
                }
                get_all_modifiers_from_effects(mods, vec![*curse.unwrap()]);
            }
            Effect::RackSizeChange(change) => {
                mods.rack_size_change += change;
            }
        }
    }
    // mods
}

pub fn get_random_effect(rng: &mut ThreadRng) -> Effect {
    let effects_list = [
        Effect::ShopDiscount,
        Effect::GainGold(3),
        Effect::VowelMultIncrease(3),
        Effect::ConsonantMultIncrease(3),
        Effect::ConsonantMultMultIncrease {
            mult: 1,
            curse: Some(Box::new(Effect::WordSizeChange(-1))),
        },
        Effect::ConsonantMultMultIncrease {
            mult: 1,
            curse: Some(Box::new(Effect::RackSizeChange(-1))),
        },
        Effect::VowelMultMultIncrease {
            mult: 1,
            curse: Some(Box::new(Effect::WordSizeChange(-1))),
        },
        Effect::VowelMultMultIncrease {
            mult: 1,
            curse: Some(Box::new(Effect::RackSizeChange(-1))),
        },
    ];
    effects_list[rng.gen::<usize>() % effects_list.len()].clone()
}

pub fn lowercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().chain(c).collect(),
    }
}

pub fn get_effect_description(effect: &Effect) -> String {
    match effect {
        Effect::ShopDiscount => "Get a shop discount".into(),
        Effect::WordSizeChange(change) => format!("Gain +{change} word count"),
        Effect::RackSizeChange(change) => format!("Gain +{change} rack size"),
        Effect::VowelMultIncrease(mult) => format!("Gain +{mult} vowel mult"),
        Effect::VowelMultMultIncrease { mult, curse } => {
            format!("Gain x{mult} vowel mult{}", explain_curse(curse))
        }
        Effect::ConsonantMultIncrease(mult) => format!("Gain +{mult} consonant mult"),
        Effect::ConsonantMultMultIncrease { mult, curse } => {
            format!("Gain x{mult} consonant mult{}", explain_curse(curse))
        }
        Effect::GainGold(gold) => format!("Gain +{gold} gold"),
        // Effect::DoubleLetterScore => "gain ",
        // Effect::DoubleWordScore => "needs description",
        // Effect::TripleLetterScore => "needs description",
        // Effect::TripleWordScore => "needs description",
        // Effect::BlankTile => "needs description",
    }
}

pub fn explain_curse(effect: &Option<Box<Effect>>) -> String {
    if let Some(effect) = effect {
        match effect.as_ref() {
            &Effect::WordSizeChange(change) => format!(". [Curse] {change} word count"),
            &Effect::RackSizeChange(change) => format!(". [Curse] {change} rack size"),
            _ => todo!(),
        }
    } else {
        "".into()
    }
}
