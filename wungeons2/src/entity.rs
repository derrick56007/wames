use std::collections::{HashMap, HashSet};

use crate::{
    components::{get_default_component, Component},
    state::State,
};

pub fn new_entity(entity_id_counter: &mut usize, components: Vec<Component>) -> Entity {
    let id = *entity_id_counter;
    *entity_id_counter += 1;
    Entity {
        id,
        components: components.clone(),
        component_index: HashMap::from_iter(
            components
                .iter()
                .enumerate()
                .map(|(i, c)| (get_default_component(c), i)),
        ),
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub components: Vec<Component>,
    pub component_index: HashMap<Component, usize>,
}

#[macro_export]
macro_rules! get_component {
    ($target: expr, $pat: path) => {{
        if let Some($pat(a)) = $target.get_component($pat(None)) {
            // #1
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}

impl Entity {
    pub fn contains_component(&self, component: &Component) -> bool {
        self.component_index.contains_key(component)
    }
    // pub fn set_component(&mut self, component: Component) {
    //     self.components[self.component_index[&get_default_component(&component)]] =
    //         component.clone();
    // }
    pub fn get_component(&self, component: Component) -> Option<Component> {
        if !self.component_index.contains_key(&component) {
            None
        } else {
            Some(self.components[self.component_index[&component]].clone())
        }
        // for c in &self.components {
        //     if get_default_component(c) == component {

        //         return match c {
        //             Component::Position(a) => Box::new(&a.unwrap()),
        //             Component::Render(a) => Box::new(&a.unwrap()),
        //             Component::ZIndex(a) => todo!(),
        //             Component::Wall => todo!(),
        //             Component::Room => todo!(),
        //             Component::Player => todo!(),
        //         };
        //     }
        // }
        // panic!();
    }
}

pub fn add_entity(entity: Entity, state: &mut State) {
    let entity = Box::new(entity);

    let entity_components: HashSet<Component> = HashSet::from_iter(
        entity
            .components
            .iter()
            .map(get_default_component)
            .collect::<Vec<Component>>(),
    );

    for c in &entity_components {
        let new: Vec<Component> = vec![c.clone()];
        if !state.component_map.contains_key(&new) {
            state.component_map.insert(new.clone(), HashSet::new());
        }
        state.component_map.get_mut(&new).unwrap().insert(entity.id);
    }

    for system_required_components in &state.system_components {
        if !state.component_map.contains_key(system_required_components) {
            state
                .component_map
                .insert(system_required_components.clone(), HashSet::new());
        }
        // dbg!(&entity_components, &system_required_components);
        if HashSet::from_iter(system_required_components.clone()).is_subset(&entity_components) {
            state
                .component_map
                .get_mut(system_required_components)
                .unwrap()
                .insert(entity.id);
        }
    }
    state.entities_map.insert(entity.id, entity);
}
