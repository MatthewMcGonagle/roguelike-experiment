use roguelike_experiment::components::*;
use roguelike_experiment::data::*;
use roguelike_experiment::entities::*;

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

    assert_eq!(components.component_types.get(owner), None);
    assert_eq!(components.component_types.get(owned), None);
    assert_eq!(components.owner.get(owner), None);
    assert_eq!(components.owner.get(owned), None);
    assert_eq!(components.owns.get(owner), None);
    assert_eq!(components.owns.get(owned), None);
    assert_eq!(components, Components::initialize(2, 0, 0));
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

    assert_eq!(components.component_types.get(owner), Some(Vec::from([ComponentType::Owns])).as_ref());
    assert_eq!(components.component_types.get(owned), None);
    assert_eq!(components.owner.get(owner), None);
    assert_eq!(components.owner.get(owned), None);
    assert_eq!(components.owns.get(owner), Some(Vec::from([])).as_ref());
    assert_eq!(components.owns.get(owned), None);
}
