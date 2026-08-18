use roguelike_experiment::components::*;
use roguelike_experiment::components::for_entities::*;
use roguelike_experiment::data::*;
use roguelike_experiment::entities::*;
use std::collections::HashMap;

#[test]
fn remove_works_on_owner() {
    let mut components = Components::initialize(2, 0, 0);
    let mut entities = Entities::initialize(2);

    let owner = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer::empty()).unwrap();

    let owned = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            owner: Some(owner),
            ..EntityBuffer::empty()}).unwrap();

    entities.remove(owner, &mut components);

    assert_eq!(
        components.to_maps(),
        ComponentMaps::new())
}

#[test]
fn remove_works_on_owned() {
    let mut components = Components::initialize(2, 0, 0);
    let mut entities = Entities::initialize(2);

    let owner = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer::empty()).unwrap();

    let owned = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            owner: Some(owner),
            ..EntityBuffer::empty()}).unwrap();

    entities.remove(owned, &mut components);

    assert_eq!(
        components.to_maps(),
        ComponentMaps {
            component_types: HashMap::from([
                (0, Vec::from([ComponentType::Owns]))
            ]),
            owns: HashMap::from([
                (0, Vec::from([]))
            ]),
            ..ComponentMaps::new()
        });
}
