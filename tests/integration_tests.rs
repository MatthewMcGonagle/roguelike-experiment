use roguelike_experiment::components::*;
use roguelike_experiment::containers::*;
use roguelike_experiment::data::*;
use roguelike_experiment::entities::*;
use roguelike_experiment::game_state::*;
use roguelike_experiment::queries::*;
use roguelike_experiment::systems::*;
use std::collections::HashMap;

#[test]
fn add_entity_buffer_works_for_owned() {
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

    assert_eq!(
        components.to_maps(),
        ComponentMaps {
            owner: HashMap::from([
                (owned, owner)]),
            ..ComponentMaps::new()});

    assert_eq!(
        queries,
        Queries {
            component_types: [
                (owner, vec![]),
                (owned, vec![ComponentType::Owner])
                ].into(),
            owns: [
                (owner, vec![owned])
                ].into(),
            ..Queries::initialize(2, 0, 0)});
}

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
        Queries {
            component_types: VecIndexedByEid::from_exactly(&vec![None, None]),
            owns: VecIndexedByEid::from_exactly(&vec![None]),
            ..Queries::initialize(0, 0, 0)
        });
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
            ..ComponentMaps::new()
        });
    assert_eq!(
        queries,
        Queries {
            owns: VecIndexedByEid::from_exactly(&vec![None]),
            component_types: VecIndexedByEid::from_exactly(
                &vec![Some(vec![]), None]),
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

    let mut to_kill = ToKill { values : vec![second] };

    do_killings(&mut to_kill, &mut components, &mut queries, &mut entities);

    assert_eq!(
        components.to_maps(),
        ComponentMaps {
            healths: HashMap::from([
                (first, 10),
                (third, 30)]),
            ..ComponentMaps::new()
        });

    assert_eq!(
        queries,
        Queries {
            component_types: VecIndexedByEid::from_exactly(
                &vec![
                    Some(vec![ComponentType::Health]),
                    None,
                    Some(vec![ComponentType::Health])
                    ]),
            ..Queries::initialize(0, 0, 0)});
}

#[test]
fn ai_kill_owner_decision_works() {
    let mut game_state = GameState::initialize(2, LoopState::MakeDecisions, Display::empty(), 0, 0);
    let owner = game_state.entities.add_entity_buffer(
        &mut game_state.components,
        &mut game_state.queries,
        &EntityBuffer::empty()).unwrap();
    let _ = game_state.entities.add_kill_timer(&mut game_state.components, &mut game_state.queries, 1, owner).unwrap();

    update_timers(&mut game_state.components.decision_timers, &mut game_state.decisions_ready);
    let _ =
        make_decisions(&mut game_state.decisions_ready, &mut game_state.components, &mut game_state.queries, &mut game_state.planned_actions).unwrap();
    do_actions(&mut game_state).unwrap();
    do_killings(&mut game_state.to_kill, &mut game_state.components, &mut game_state.queries, &mut game_state.entities);

    assert_eq!(
        game_state.components.to_maps(),
        ComponentMaps::new());

    assert_eq!(
        game_state.queries,
        Queries {
            component_types: VecIndexedByEid::from_exactly(&vec![None, None]),
            owns: VecIndexedByEid::from_exactly(&vec![None]),
            ..Queries::initialize(0, 0, 0)
        });
}
