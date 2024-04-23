use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io::{self, Write},
};

use chrono::{Datelike, Utc};
use rand::Rng;

const COLORS: [(&str, &str); 4] = [
    ("green", "\x1b[42m"),
    ("yellow", "\x1b[43m"),
    ("white", "\x1b[47m"),
    ("reset", "\x1b[30m"),
];

fn colorize(input: char, color: &str, colors: &HashMap<&str, &str>) -> String {
    format!("{}{input}\x1b[0m", colors[color])
}

fn get_word(daily: bool, words_vec: &[String]) -> (String, Vec<char>, HashSet<char>) {
    let word = words_vec[if daily {
        let now = Utc::now();
        ((now.year() as usize) << 2) | ((now.month() as usize) << 2) | (now.day() as usize)
    } else {
        let mut rng = rand::thread_rng();
        rng.gen::<usize>()
    } % words_vec.len()]
    .to_string();
    let word_vec: Vec<char> = word.chars().collect();
    let word_set: HashSet<char> = HashSet::from_iter(word_vec.clone());

    (word, word_vec, word_set)
}

fn main() {
    let tries = 6;
    let words_vec: Vec<String> = fs::read_to_string("/usr/share/dict/words")
        .unwrap()
        .split("\n")
        .filter_map(|s| {
            if s.len() == 5 {
                Some(s.to_lowercase())
            } else {
                None
            }
        })
        .collect();
    let words_set: HashSet<String> = HashSet::from_iter(words_vec.clone());
    let (mut word, mut word_vec, mut word_set) = get_word(
        if let Some(arg) = env::args().nth(1) {
            if arg == "daily" {
                true
            } else {
                false
            }
        } else {
            false
        },
        &words_vec,
    );

    let stdin = io::stdin();
    let input = &mut String::new();
    print!("\x1B[2J\x1B[1;1H\n");
    let mut attempts: Vec<String> = vec![];
    let colors: HashMap<&str, &str> = HashMap::from_iter(COLORS);
    let mut game_over = false;
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut used_letters: HashSet<char> = HashSet::new();
    let mut misplaced_letters: HashSet<char> = HashSet::new();
    let mut correct_letters: HashSet<char> = HashSet::new();
    let mut not_a_word = false;
    let mut game_over_text: Option<String> = None;
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("          wurdle");
        println!("==========================\n");
        // println!("{}", &word);

        if !attempts.is_empty() && !words_set.contains(attempts.last().unwrap()) {
            attempts.pop();
            not_a_word = true;
        }

        for attempt in 0..tries {
            print!("{}) ", attempt + 1);

            if attempt < attempts.len() {
                let a = &attempts[attempt];
                'inner: for (i, c) in a.chars().enumerate() {
                    if i >= word_vec.len() {
                        break 'inner;
                    }
                    if c == (&word_vec)[i] {
                        print!("{}", colorize(c, "green", &colors));
                        correct_letters.insert(c);
                    } else if word_set.contains(&c) {
                        print!("{}", colorize(c, "yellow", &colors));
                        misplaced_letters.insert(c);
                    } else {
                        print!("{}", colorize(c, "reset", &colors));
                        used_letters.insert(c);
                    }
                }
                print!("\n\n");
                if *a == word {
                    game_over_text = Some("you won! restart? (y/n)\n".to_string());
                    game_over = true;
                } else if attempt + 1 == tries {
                    game_over_text =
                        Some(format!("you lost! the word was '{word}'\nrestart? (y/n)\n"));
                    game_over = true;
                }
            } else {
                println!("_____\n");
            }
        }
        if not_a_word {
            println!("'{input}' is not a word! please try again\n");
            not_a_word = false;
        }
        for c in LETTERS.chars() {
            print!(
                "{}",
                colorize(
                    c,
                    if correct_letters.contains(&c) {
                        "green"
                    } else if misplaced_letters.contains(&c) {
                        "yellow"
                    } else if used_letters.contains(&c) {
                        "reset"
                    } else {
                        "white"
                    },
                    &colors
                )
            )
        }
        println!("\n");
        if let Some(ref text) = game_over_text {
            println!("{text}");
        }
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(input).unwrap();
        input.pop();

        if game_over == true {
            if input == "y" {
                game_over = false;
                (word, word_vec, word_set) = get_word(false, &words_vec);
                attempts.clear();
                used_letters.clear();
                misplaced_letters.clear();
                correct_letters.clear();
                game_over_text = None;

            } else if input == "n" {
                return;
            }
        } else {
            attempts.push(input.clone());
        }
    }
}
