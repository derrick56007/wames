use std::{
    collections::HashSet,
    io::{self, Read},
};

use wurdle::{play, wurdle_words};

fn main() {
    let tries = 6;
    let available_letters: HashSet<char> = HashSet::from_iter("".chars());
    let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();
    play(tries, available_letters, words_vec, None);
}
