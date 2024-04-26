use std::collections::{HashMap, HashSet};

use crate::{
    components::{get_default_component, Component},
    state::State,
};

pub fn new_entity(entity_id_counter: &mut usize, components: Vec<Component>) -> Entity {
    let id = entity_id_counter.clone();
    *entity_id_counter = *entity_id_counter + 1;
    Entity {
        id,
        components: HashSet::from_iter(components),
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub components: HashSet<Component>,
}

pub fn add_entity(
    entity: Entity,
    entities: &mut HashMap<usize, Box<Entity>>,
    component_map: &mut HashMap<Vec<Component>, HashSet<usize>>,
    systems: &[(
        fn(&HashSet<usize>, &mut State, &mut HashMap<usize, Box<Entity>>),
        Vec<Component>,
    )],
) {
    let entity = Box::new(entity);

    for (_, system_required_components) in systems {
        let entity_components: HashSet<Component> = HashSet::from_iter(
            entity
                .components
                .iter()
                .map(|c| get_default_component(c))
                .collect::<Vec<Component>>(),
        );

        if !component_map.contains_key(system_required_components) {
            component_map.insert(system_required_components.clone(), HashSet::new());
        }
        // dbg!(&entity_components, &system_required_components);
        if HashSet::from_iter(system_required_components.clone()).is_subset(&entity_components) {
            component_map
                .get_mut(system_required_components)
                .unwrap()
                .insert(entity.id);
        }


    }
    entities.insert(entity.id, entity);
}
