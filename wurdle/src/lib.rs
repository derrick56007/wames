pub mod wurdle_words;

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

use chrono::{Datelike, Utc};
use rand::Rng;

const COLORS: [(&str, &str); 4] = [
    ("green", "\x1b[42m"),
    ("yellow", "\x1b[43m"),
    ("white", "\x1b[47m"),
    ("reset", "\x1b[0m"),
];

fn colorize(input: String, color: &str, colors: &HashMap<&str, &str>) -> String {
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

pub fn play(tries: usize, available_letters: HashSet<char>, words_vec: Vec<String>) -> bool {
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
    let args = env::args().collect::<Vec<String>>();
    let mut db = Stats::read();
    let stdin = io::stdin();
    let mut input = String::new();
    print!("\x1B[2J\x1B[1;1H\n");
    let mut attempts: Vec<String> = vec![];
    let colors: HashMap<&str, &str> = HashMap::from_iter(COLORS);
    let mut game_over = false;
    const LETTERS: &str = "Q W E R T Y U I O P\n A S D F G H J K L\n  Z X C V B N M";
    let mut used_letters: HashSet<char> = HashSet::new();
    let mut misplaced_letters: HashSet<char> = HashSet::new();
    let mut correct_letters: HashSet<char> = HashSet::new();
    let mut not_a_word = false;
    let mut game_over_text: Option<String> = None;
    let mut won_attempt: Option<usize> = None;
    let mut won = false;

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("          wurdle");
        println!("==========================\n");
        println!("{}", &word);

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
                        print!("{}", colorize(c.into(), "green", &colors));
                        correct_letters.insert(c);
                    } else if word_set.contains(&c) {
                        print!("{}", colorize(c.into(), "yellow", &colors));
                        misplaced_letters.insert(c);
                    } else {
                        print!("{}", colorize(c.into(), "reset", &colors));
                        used_letters.insert(c);
                    }
                }
                print!("\n\n");
                if *a == word {
                    game_over_text = Some("     you won!".to_string());
                    game_over = true;
                    db.won(attempt + 1);
                    db.save();
                    won_attempt = Some(attempt + 1);
                    won = true;
                } else if attempt + 1 == tries {
                    game_over_text = Some(format!("you lost! the word was '{word}'"));
                    game_over = true;
                    won_attempt = None;
                    won = false;
                    db.save();
                    db.lost();
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
                    c.into(),
                    if correct_letters.contains(&c) {
                        "green"
                    } else if misplaced_letters.contains(&c) {
                        "yellow"
                    } else if used_letters.contains(&c)
                        || c == ' '
                        || (!available_letters.is_empty() && !available_letters.contains(&c))
                    {
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
            let scale = 2;
            let stats = format!(
                "    STATISTICS
==================
Played:\t\t{}
Win-rate:\t{}%
Current Streak:\t{}
Max Streak:\t{}

GUESS DISTRIBUTION
==================
1 {}{}
2 {}{}
3 {}{}
4 {}{}
5 {}{}
6 {}{}\n\n  restart? (y/n)",
                db.played,
                f64::trunc((db.won as f64) / (db.played as f64) * 100.0 * 100.0) / 100.0,
                db.streak,
                db.maxstreak,
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 1 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_1 * scale),
                colorize(
                    db.dist_1.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 1 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 2 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_2 * scale),
                colorize(
                    db.dist_2.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 2 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 3 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_3 * scale),
                colorize(
                    db.dist_3.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 3 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 4 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_4 * scale),
                colorize(
                    db.dist_4.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 4 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 5 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_5 * scale),
                colorize(
                    db.dist_5.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 5 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
                colorize(
                    ' '.into(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 6 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                )
                .repeat(db.dist_6 * scale),
                colorize(
                    db.dist_6.to_string(),
                    if let Some(won_attempt) = won_attempt {
                        if won_attempt == 6 {
                            "green"
                        } else {
                            "white"
                        }
                    } else {
                        "white"
                    },
                    &colors
                ),
            );
            println!("\n{text}\n\n{stats}\n");
        }
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        input.pop();

        let bytes = input.as_bytes();
        if bytes.len() > 3 && &bytes[0..2] == &[27, 91] {
            input = String::from_utf8_lossy(&bytes[3..]).to_string();
        }

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
                return won;
            }
        } else {
            attempts.push(input.to_uppercase());
        }
    }
}

struct Stats {
    pub played: usize,
    pub won: usize,
    pub streak: usize,
    pub maxstreak: usize,
    pub dist_1: usize,
    pub dist_2: usize,
    pub dist_3: usize,
    pub dist_4: usize,
    pub dist_5: usize,
    pub dist_6: usize,
}

const DB_FILE: &str = "./.wurdle";

impl Stats {
    pub fn read() -> Self {
        let lines = fs::read_to_string(DB_FILE);

        if let Ok(lines) = lines {
            let lines = lines.split("\n").collect::<Vec<&str>>();
            Self {
                played: lines[0].parse().unwrap(),
                won: lines[1].parse().unwrap(),
                streak: lines[2].parse().unwrap(),
                maxstreak: lines[3].parse().unwrap(),
                dist_1: lines[4].parse().unwrap(),
                dist_2: lines[5].parse().unwrap(),
                dist_3: lines[6].parse().unwrap(),
                dist_4: lines[7].parse().unwrap(),
                dist_5: lines[8].parse().unwrap(),
                dist_6: lines[9].parse().unwrap(),
            }
        } else {
            Self {
                played: 0,
                won: 0,
                streak: 0,
                maxstreak: 0,
                dist_1: 0,
                dist_2: 0,
                dist_3: 0,
                dist_4: 0,
                dist_5: 0,
                dist_6: 0,
            }
        }
    }

    pub fn won(&mut self, attempt: usize) {
        self.won += 1;
        self.played += 1;
        self.streak += 1;
        if self.streak > self.maxstreak {
            self.maxstreak = self.streak;
        }
        if attempt == 1 {
            self.dist_1 += 1;
        } else if attempt == 2 {
            self.dist_2 += 1;
        } else if attempt == 3 {
            self.dist_3 += 1;
        } else if attempt == 4 {
            self.dist_4 += 1;
        } else if attempt == 5 {
            self.dist_5 += 1;
        } else if attempt == 6 {
            self.dist_6 += 1;
        }
    }

    pub fn lost(&mut self) {
        self.streak = 0;
        self.played += 1;
    }

    pub fn save(&self) {
        let mut output = if Path::new(DB_FILE).exists() {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(Path::new(DB_FILE))
                .unwrap()
        } else {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(Path::new(DB_FILE))
                .unwrap()
        };
        write!(
            output,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.played,
            self.won,
            self.streak,
            self.maxstreak,
            self.dist_1,
            self.dist_2,
            self.dist_3,
            self.dist_4,
            self.dist_5,
            self.dist_6
        )
        .unwrap();
    }
}
