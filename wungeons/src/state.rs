use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use device_query::{DeviceState, Keycode};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Component, Item, Position, Rect},
    entity::Entity,
    event::Event,
    rooms::RoomType,
};

pub struct State {
    pub grid_size: Rect,
    pub events: Vec<Event>,
    pub change_events: Vec<Component>,
    pub last_pressed_key: Option<Keycode>,
    pub device_state: DeviceState,
    pub rng: ThreadRng,
    pub entity_id_counter: usize,

    pub entities_map: HashMap<usize, Box<Entity>>,
    pub component_map: HashMap<Vec<Component>, HashSet<usize>>,
    pub system_components: Vec<Vec<Component>>,
    pub rooms: Vec<(Rect, Position, RoomType)>,
    pub available_letters: HashSet<char>,
    pub full_loop_duration: Option<Duration>,
    pub letters_remaining: Vec<char>,
    pub items: Vec<Item>,
    // pub systems: Vec<(fn(&mut State, &Vec<Component>), Vec<Component>, bool)>,
}

impl State {
    pub fn new(grid_size: Rect, system_components: Vec<Vec<Component>>) -> Self {
        // let mut rng = ;

        let mut n = Self {
            grid_size,
            events: vec![],
            last_pressed_key: None,
            device_state: DeviceState::new(),
            rng: rand::thread_rng(),
            entity_id_counter: 0,

            entities_map: HashMap::new(),
            component_map: HashMap::new(),
            system_components,
            rooms: Vec::new(),
            change_events: Vec::new(),
            letters_remaining: Vec::new(),
            // system_components: HashSet::new(),
            // systems: get_systems(),
            available_letters: HashSet::new(),
            full_loop_duration: None,
            items: Vec::new(),
        };
        n.refresh();
        n
    }

    pub fn refresh(&mut self) {
        let mut available_letters: HashSet<char> =
            HashSet::from_iter("aeuio".to_uppercase().chars());
        let mut letters_remaining = "abcdefghijklmnopqrstuvwxys".to_uppercase().to_string();
        let starting_letters_count = 10;
        for c in &available_letters {
            letters_remaining = letters_remaining.replace(*c, "");
        }
        let mut letters_remaining = letters_remaining.chars().collect::<Vec<char>>();

        while available_letters.len() < starting_letters_count {
            let new_letter = letters_remaining[self.rng.gen::<usize>() % letters_remaining.len()];
            available_letters.insert(new_letter);
            letters_remaining = String::from_iter(letters_remaining)
                .replace(new_letter, "")
                .chars()
                .collect::<Vec<char>>()
        }
        self.available_letters = available_letters;
        self.letters_remaining = letters_remaining;
    }

    pub fn get_entities(&self, components: &[Component]) -> &HashSet<usize> {
        &self.component_map[components]
    }

    pub fn get_unchosen_letters_if_possible(&mut self, n: usize) -> Vec<char> {
        let mut available_letters: HashSet<char> = HashSet::from_iter("".chars());
        let mut letters_remaining = self.letters_remaining.clone();

        while available_letters.len() < n {
            let new_letter = letters_remaining[self.rng.gen::<usize>() % letters_remaining.len()];
            if !available_letters.contains(&new_letter) {
                available_letters.insert(new_letter);
                letters_remaining = String::from_iter(letters_remaining)
                    .replace(new_letter, "")
                    .chars()
                    .collect::<Vec<char>>()
            }
        }

        available_letters.iter().copied().collect()
    }

    pub fn add_letter(&mut self, c: char) {
        self.available_letters.insert(c);
        self.letters_remaining = String::from_iter(self.letters_remaining.iter())
            .replace(c, "")
            .chars()
            .collect();
    }

    pub fn remove_entity(&mut self, id: usize) {
        for (_, v) in self.component_map.iter_mut() {
            v.remove(&id);
        }

        self.entities_map.remove(&id);
    }
}
