use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use fnv::{FnvHashMap, FnvHashSet};
// use device_query::{DeviceState, Keycode};
use rand::{rngs::ThreadRng, Rng};
use winit::keyboard::{KeyCode, SmolStr};


use crate::{
    components::{get_default_component, Component,  Position, Rect}, effects::AllModifiers, entity::Entity, event::Event, items::Item, letters::{get_starting_tile_points, get_starting_tiles}, render::Show, rooms::RoomType
};

pub struct State {
    pub grid_size: Rect,
    pub events: Vec<Event>,
    pub keys_pressed: FnvHashMap<KeyCode, String>,
    pub change_events: Vec<Component>,
    // pub last_pressed_keys: FnvHashSet<KeyCode>,
    // pub device_state: DeviceState,
    pub rng: ThreadRng,
    pub entity_id_counter: usize,
    pub name: String,

    pub entities_map: FnvHashMap<usize, Box<Entity>>,
    pub component_map: FnvHashMap<Vec<Component>, FnvHashSet<usize>>,
    pub system_components: Vec<Vec<Component>>,
    pub rooms: Vec<(Rect, Position, RoomType)>,
    pub hallways: Vec<(Position, Position)>,
    pub available_letters: Vec<char>,
    pub update_loop_duration: Duration,
    pub render_loop_duration: Duration,
    // pub letters_remaining: Vec<char>,
    pub items: Vec<Item>,
    pub gold: usize,
    pub fog_enabled: bool,
    pub dialogue_input: String,
    empty_entites_set: FnvHashSet<usize>,
    // pub show_deck: bool,
    pub floor: usize,
    pub mods: AllModifiers,
    pub tile_points: FnvHashMap<char, usize>,
    pub no_keys_pressed: bool,
    pub show: Show,
    // pub step_counter: usize,
    // pub systems: Vec<(fn(&mut State, &Vec<Component>), Vec<Component>, bool)>,
}

impl State {
    pub fn new(grid_size: Rect, system_components: Vec<Vec<Component>>) -> Self {
        // let mut rng = ;

        let mut n = Self {
            // show_deck: false,
            grid_size,
            events: vec![],
            show: Show::None,
            keys_pressed: FnvHashMap::default(),
            // last_pressed_keys: FnvHashSet::default(),
            // device_state: DeviceState::new(),
            rng: rand::thread_rng(),
            entity_id_counter: 0,

            entities_map: FnvHashMap::default(),
            component_map: FnvHashMap::default(),
            system_components,
            rooms: Vec::new(),
            hallways: Vec::new(),
            change_events: Vec::new(),
            // letters_remaining: Vec::new(),
            // step_counter: 0,
            // system_components: FnvHashSet::default(),
            // systems: get_systems(),
            available_letters: Vec::new(),
            no_keys_pressed: true,
            update_loop_duration: Duration::ZERO,
            render_loop_duration: Duration::ZERO,

            items: Vec::new(),
            gold: 0,
            fog_enabled: true,
            dialogue_input: "".to_string(),
            name: "".to_string(),
            empty_entites_set: FnvHashSet::default(),
            floor: 1,
            mods: AllModifiers::default(),
            tile_points: get_starting_tile_points(),
        };
        n.refresh_tiles();
        n
    }

    pub fn refresh_tiles(&mut self) {
        // let available_letters: FnvHashSet<char> =
        //     FnvHashSet::from_iter("aerotlisncuyd".to_uppercase().chars());
        // let mut letters_remaining = "abcdefghijklmnopqrstuvwxyz".to_uppercase().to_string();
        // // let starting_letters_count = 10;
        // for c in &available_letters {
        //     letters_remaining = letters_remaining.replace(*c, "");
        // }
        // let letters_remaining = letters_remaining.chars().collect::<Vec<char>>();

        // while available_letters.len() < starting_letters_count {
        //     let new_letter = letters_remaining[self.rng.gen::<usize>() % letters_remaining.len()];
        //     available_letters.insert(new_letter);
        //     letters_remaining = String::from_iter(letters_remaining)
        //         .replace(new_letter, "")
        //         .chars()
        //         .collect::<Vec<char>>()
        // }
        self.available_letters = get_starting_tiles();
        // self.letters_remaining = letters_remaining;
    }

    pub fn get_entities(&self, components: &[Component]) -> &FnvHashSet<usize> {
        self.component_map.get(components).unwrap_or(&self.empty_entites_set)
    }

    // pub fn get_unchosen_letters_if_possible(&mut self, n: usize) -> Vec<char> {
    //     let mut available_letters: FnvHashSet<char> = FnvHashSet::from_iter("".chars());
    //     let mut letters_remaining = self.letters_remaining.clone();

    //     while available_letters.len() < n {
    //         let new_letter = letters_remaining[self.rng.gen::<usize>() % letters_remaining.len()];
    //         if !available_letters.contains(&new_letter) {
    //             available_letters.insert(new_letter);
    //             letters_remaining = String::from_iter(letters_remaining)
    //                 .replace(new_letter, "")
    //                 .chars()
    //                 .collect::<Vec<char>>()
    //         }
    //     }

    //     available_letters.iter().copied().collect()
    // }

    // pub fn add_letter(&mut self, c: char) {
    //     self.available_letters.push(c);
    //     self.letters_remaining = String::from_iter(self.letters_remaining.iter())
    //         .replace(c, "")
    //         .chars()
    //         .collect();
    // }

    pub fn remove_entity(&mut self, id: usize) {
        for (_, v) in self.component_map.iter_mut() {
            v.remove(&id);
        }

        self.entities_map.remove(&id);
    }

    pub fn set_component(&mut self, id: usize, component: Component) {
        let e = &mut self.entities_map.get_mut(&id).unwrap();
        if e.components[e.component_index[&get_default_component(&component)]] != component {
            self.events.push(Event::ComponentChanged(component.clone()));
        }

        e.components[e.component_index[&get_default_component(&component)]] = component.clone();
    }

    // pub fn remove_component(&mut self, component: Component) {
    //     let c = get_default_component(c);

    // }

    pub fn remove_all_by_component(&mut self, component: Component) {
        let key = vec![component];
        if !self.component_map.contains_key(&key) {
            return;
        }

        let entities = self.component_map[&key].clone();
        for e in entities {
            self.remove_entity(e);
        }
    }

    pub fn apply_modifiers(&mut self, modifiers: &mut AllModifiers) {
        self.mods.add(modifiers);
    }
}
