use std::{
    collections::{HashMap, HashSet},
    io::{self, stdin, Read},
};

use wurdle::{play, wurdle_words};

fn main() {

    
    // stdin().read_line(&mut "".to_string());

    let tries = 6;
    let available_letters: HashSet<char> = HashSet::from_iter("".chars());
    let words_vec: Vec<String> = wurdle_words::WURDLE_WURDS
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();

    // let mut hashm = HashMap::<char, usize>::new();
    // for word in &words_vec {
    //     for c in word.chars() {
    //         if !hashm.contains_key(&c) {
    //             hashm.insert(c, 0);
    //         }
    //         hashm.insert(c, hashm[&c] + 1);
    //     }
    // }
    // let mut k = hashm.iter().map(|(c, u)| (*c, *u)).collect::<Vec<(char, usize)>>();
    // k.sort_by(|a, b| a.1.cmp(&b.1));
    // println!("{:#?}", k);
    play(tries, available_letters, words_vec, None, false);
}
