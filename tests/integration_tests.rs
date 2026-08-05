use roguelike_experiment::components::*;
use roguelike_experiment::data::*;
use roguelike_experiment::entities::*;

#[test]
fn remove_works_on_owner() {
    let mut components = Components::initialize(2, 0, 0);
    let mut entities = Entities::initialize(2);

    let owner = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            ai: None,
            alignment: None,
            blocking: None,
            coords: None,
            decision_timer: None,
            health: None,
            owner: None,
            render: None,
            state: None}).unwrap();

    let owned = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            ai: None,
            alignment: None,
            blocking: None,
            coords: None,
            decision_timer: None,
            health: None,
            owner: Some(owner),
            render: None,
            state: None}).unwrap();

    entities.remove(owner, &mut components);

    assert_eq!(4, 4);
}

#[test]
fn remove_works_on_owned() {
    let mut components = Components::initialize(2, 0, 0);
    let mut entities = Entities::initialize(2);

    let owner = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            ai: None,
            alignment: None,
            blocking: None,
            coords: None,
            decision_timer: None,
            health: None,
            owner: None,
            render: None,
            state: None}).unwrap();

    let owned = entities.add_entity_buffer(
        &mut components,
        &EntityBuffer {
            ai: None,
            alignment: None,
            blocking: None,
            coords: None,
            decision_timer: None,
            health: None,
            owner: Some(owner),
            render: None,
            state: None}).unwrap();

    entities.remove(owned, &mut components);

    assert_eq!(4, 4);
}
