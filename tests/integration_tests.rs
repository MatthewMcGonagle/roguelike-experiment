use roguelike_experiment::components::*;
use roguelike_experiment::containers::*;
use roguelike_experiment::data::*;
use roguelike_experiment::entities::*;
use roguelike_experiment::game_state::*;
use roguelike_experiment::queries::*;
use roguelike_experiment::systems::*;
use std::collections::HashMap;

#[test]
fn remove_works_on_owner() {
    let mut components = Components::initialize(2);
    let mut entities = Entities::initialize(2);
    let mut queries = Queries::initialize(2, 0, 0);

    let owner = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer::empty()).unwrap();

    let _ = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer {
            owner: Some(owner),
            ..EntityBuffer::empty()}).unwrap();

    entities.remove(owner, &mut components, &mut queries);

    assert_eq!(
        components.to_maps(),
        ComponentMaps::new());

    assert_eq!(
        queries,
        Queries::initialize(0, 0, 0));
}

#[test]
fn remove_works_on_owned() {
    let mut components = Components::initialize(2);
    let mut entities = Entities::initialize(2);
    let mut queries = Queries::initialize(2, 0, 0);

    let owner = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer::empty()).unwrap();

    let owned = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer {
            owner: Some(owner),
            ..EntityBuffer::empty()}).unwrap();

    entities.remove(owned, &mut components, &mut queries);

    assert_eq!(
        components.to_maps(),
        ComponentMaps {
            // component_types: HashMap::from([
            //     (0, Vec::from([ComponentType::Owns]))
            // ]),
            // owns: HashMap::from([
            //     (0, Vec::from([]))
            // ]),
            ..ComponentMaps::new()
        });
    assert_eq!(
        queries,
        Queries {
            component_types: VecIndexedByEid::from_exactly(
                &Vec::from([Some(Vec::from([])), None])),
            ..Queries::initialize(0, 0, 0)
        });
}

#[test]
fn do_killings_works() {
    let mut components = Components::initialize(3);
    let mut entities = Entities::initialize(3);
    let mut queries = Queries::initialize(2, 0, 0);

    let first = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer {
            health: Some(10),
            ..EntityBuffer::empty()}).unwrap();

    let second = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer {
            health: Some(20),
            ..EntityBuffer::empty()}).unwrap();

    let third = entities.add_entity_buffer(
        &mut components,
        &mut queries,
        &EntityBuffer {
            health: Some(30),
            ..EntityBuffer::empty()}).unwrap();

    let mut to_kill = ToKill { values : Vec::from([second]) };

    do_killings(&mut to_kill, &mut components, &mut queries, &mut entities);

    assert_eq!(
        components.to_maps(),
        ComponentMaps {
            // component_types: HashMap::from([
            //     (first, Vec::from([ComponentType::Health])),
            //     (third, Vec::from([ComponentType::Health]))]),
            healths: HashMap::from([
                (first, 10),
                (third, 30)]),
            ..ComponentMaps::new()
        });

    assert_eq!(
        queries,
        Queries {
            component_types: VecIndexedByEid::from_exactly(
                &Vec::from([
                    Some(Vec::from([ComponentType::Health])),
                    None,
                    Some(Vec::from([ComponentType::Health]))
                    ])),
            ..Queries::initialize(0, 0, 0)});
}
