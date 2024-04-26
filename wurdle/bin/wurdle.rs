use std::{collections::HashSet, io::{self, Read}};

use wurdle::{play, wurdle_words};



fn main() {
    let tries = 6;
    let available_letters: HashSet<char> = HashSet::from_iter("".chars());
    let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
        .split("\n")
        .map(|s| s.to_uppercase())
        .filter(|word| {
            if !available_letters.is_empty() {
                word.chars().all(|c| available_letters.contains(&c))
            } else {
                true
            }
        })
        .collect();
    play(tries, available_letters, words_vec);
}
