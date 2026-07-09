mod free_eids;

use crate::components::*;
use crate::data::*;
use crate::state_storage::*;
use free_eids::FreeEids;
use std::collections::HashMap;

pub struct Entities {
    free_ids: FreeEids,
    pub active_ids: Vec<usize>
}

impl Entities {
    pub fn initialize(free_ids_allocation_size: usize) -> Entities {
        Entities {
            free_ids: FreeEids::initialize(free_ids_allocation_size),
            active_ids: Vec::with_capacity(free_ids_allocation_size)
        }
    }

    fn free_most_recent_id(&mut self) -> Result<(), Errors> {
        let e_id = self.active_ids.pop().ok_or(Errors::UnexpectedlyEmpty)?;
        self.free_ids.push(e_id);
        Ok(())
    }

    fn ensure_coords_free_when_needed(components: &Components, entity: &EntityBuffer) -> Result<(), Errors> {
        match entity.blocking {
            Some(BlockingType::Movement) => match entity.coords.as_ref() {
                Some(c) => {
                    let space_data = components.coords_query.get(c.x, c.y)?;
                    if let SpaceData::Empty = space_data {
                        Ok(())
                    } else { Err(Errors::SpaceAlreadyNonempty) }
                },
                None => Ok(())
            },
            None => Ok(())
        }
    }

    pub fn ensure_owner_exists(components: &mut Components, entity: &EntityBuffer) -> Result<(), Errors> {
        match entity.owner {
            Some(x) => {
                if components.component_types.get(x).is_some() {
                    Ok(())
                } else { Err(Errors::MissingExpectedEid) }
            },
            None => Ok(())
        }
    }

    pub fn add_to_coords_query_when_needed(components: &mut Components, entity: &EntityBuffer, e_id: usize) -> Result<Option<ComponentType>, Errors> {
        let maybe_space_data = match entity.blocking {
            Some(BlockingType::Movement) => entity.coords.as_ref()
                .map(|c|
                    components.coords_query.add(c.x, c.y, SpaceData::HasEid(e_id)))
                .transpose()?,
            None => None
        };
        Ok(maybe_space_data)
    }

    pub fn add_to_owner(components: &mut Components, owner_id: usize, e_id: usize) -> () {
        match components.owns.get_mut(owner_id) {
            Some(xs) => xs.push(e_id),
            None => {
                components.owns.add(e_id, Vec::from([e_id]));
                let _ = components.component_types.get_mut(e_id).map(|ts| ts.push(ComponentType::Owns));
            }
        }
    }

    pub fn add_entity_buffer(&mut self, components: &mut Components, entity: &EntityBuffer) -> Result<usize, Errors> {
        Entities::ensure_coords_free_when_needed(components, entity)?;
        Entities::ensure_owner_exists(components, entity)?;

        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        entity.owner.map(|o| Entities::add_to_owner(components, o, e_id));
        let maybe_space_component = Entities::add_to_coords_query_when_needed(components, entity, e_id)?;

        let components_added = Vec::from([
            entity.ai.as_ref().map(|ai| components.ais.add(e_id, ai.clone())),
            entity.alignment.as_ref().map(|a| components.alignments.add(e_id, a.clone())),
            entity.blocking.as_ref().map(|b| components.blocking.add(e_id, b.clone())),
            entity.coords.as_ref().map(|cs| components.coords.add(e_id, cs.clone())),
            maybe_space_component,
            entity.decision_timer.as_ref().map(|dt| components.decision_timers.add(e_id, dt.clone())),
            entity.health.as_ref().map(|h| components.healths.add(e_id, h.clone())),
            entity.owner.map(|o| components.owner.add(e_id, o)),
            entity.render.as_ref().map(|r| components.renders.add(e_id, r.clone())),
            entity.state.as_ref().map(|s| components.states.add(e_id, s.clone()))
        ]).into_iter().flatten().collect();
        components.component_types.add(e_id, components_added);

        Ok(e_id)
    }

    pub fn add_state_storage(&mut self, state_store: &StateStorage) -> Result<(), Errors> {
        let mut sid_to_eid: HashMap<usize, usize> = HashMap::new();
        let mut updated_owner = EntityBuffer::empty();

        for entity in &state_store.entities {
            // first change the owner id to correct eid when necessary.
            let eid_version =
                if let Some(owner_sid) = entity.entity.owner {
                    let owner_eid = sid_to_eid.get(&owner_sid).ok_or(Errors::MissingExpectedEid)?;
                    updated_owner = entity.entity.clone();
                    updated_owner.owner = Some(*owner_eid);
                    &updated_owner
                } else {
                    &entity.entity
                };
        }
        Ok(())
    }

    pub fn add_wall_block(&mut self, components: &mut Components, coords: Coordinates, render: Render) -> Result<usize, Errors> {
        let entity_data = EntityBuffer {
            ai: None,
            alignment: None,
            blocking: Some(BlockingType::Movement),
            coords: Some(coords),
            decision_timer: None,
            health: None,
            owner: None,
            render: Some(render),
            state: None
        };

        self.add_entity_buffer(components, &entity_data)
    }

    pub fn add_timed_square(
        &mut self, components: &mut Components, coords: Coordinates, time_size: u32, ai: Ai, alignment: AlignmentType, health: i32, render: Render
    ) -> Result<usize, Errors> {
        let entity_data = EntityBuffer {
            ai: Some(ai),
            alignment: Some(alignment),
            blocking: Some(BlockingType::Movement),
            coords: Some(coords),
            decision_timer: Some(Timer { time: time_size, reset: time_size }),
            health: Some(health),
            owner: None,
            render: Some(render),
            state: None
        };

        self.add_entity_buffer(components, &entity_data)
    } 

    pub fn add_timed_square_creator(&mut self, components: &mut Components, coords: Coordinates, time_size: u32) -> Result<(), Errors> {
        let entity_data = EntityBuffer {
            ai: Some(Ai::AddAvailableSquare),
            alignment: None,
            blocking: None,
            coords: Some(coords),
            decision_timer: Some(Timer { time: time_size, reset: time_size }),
            health: None,
            owner: None,
            render: None,
            state: Some(0)
        };

        let _ = self.add_entity_buffer(components, &entity_data)?;
        Ok(())
    }

    pub fn add_kill_timer(&mut self, components: &mut Components, time_size: u32, target_e_id: usize) -> Result<(), Errors> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        let components_added = Vec::from([
            components.decision_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            components.ais.add(e_id, Ai::Kill),
            components.owner.add(e_id, target_e_id)
        ]);
        components.component_types.add(e_id, components_added);

        // Need to handle the target too.
        let target_component = components.owns.add(target_e_id, Vec::from([e_id]));
        components.component_types.push(target_e_id, target_component)
    }

    pub fn remove(&mut self, e_id: usize, components: &mut Components) {
        // Should only be one element.
        let inds: Vec<usize> =
            self.active_ids.iter().enumerate()
                .map(|(i, id)| (i, *id))
                .filter(|(_, id)| *id == e_id)
                .map(|(i, _)| i)
                .collect();
        // Make sure we only make the e_id free if we actually deactivated it.
        if let Some(i) = inds.get(0) {
            self.active_ids.swap_remove(*i);
            self.free_ids.push(e_id);
        }

        // To avoid borrow checker difficulties, let us just collect a list. This will also help us
        // avoid any dropped linkage errors created by deletion process. 
        let owns: Vec<usize> = components.owns.get(e_id).into_iter().flat_map(|x| x.clone()).collect();
        for x in owns {
            self.remove(x, components);
        }

        if let Some(&owner) = components.owner.get(e_id) {
            let maybe_owner_entities = components.owns.get_mut(owner);
            // map() will consume the value, but &mut is not copyable. So let's get an immutable
            // copy, works better with map().
            let maybe_imm_borrow: Option<& Vec<usize>> = match maybe_owner_entities {
                Some(ref xs) => Some(xs),
                None => None
            };

            let maybe_pos = maybe_imm_borrow
                .map(|xs| xs.iter().position(|x| *x == e_id))
                .flatten();

            match maybe_pos {
                Some(pos) => maybe_owner_entities.map(|owner_entities| owner_entities.swap_remove(pos)),
                None => None
            };
        }

        components.component_types.get(e_id).map(
            |c_types| for c_type in c_types { 
                match c_type {
                    ComponentType::ComponentTypeList => (),
                    ComponentType::Coordinates => {
                        components.coords.get(e_id).map(|c|
                            components.coords_query.get_mut(c.x, c.y).map(|s| *s = SpaceData::Empty)
                        );
                        components.coords.remove(e_id);
                    },
                    ComponentType::CoordinatesQuery => (),
                    ComponentType::Blocking => components.blocking.remove(e_id),
                    ComponentType::DecisionTimer => components.decision_timers.remove(e_id),
                    ComponentType::Ai => components.ais.remove(e_id),
                    ComponentType::State => components.states.remove(e_id),
                    ComponentType::Render => components.renders.remove(e_id),
                    ComponentType::Owns => components.owns.remove(e_id),
                    ComponentType::Owner => components.owner.remove(e_id),
                    ComponentType::Alignment => components.alignments.remove(e_id),
                    ComponentType::Health => components.healths.remove(e_id)
                }
            }
        );

        components.component_types.remove(e_id);
    }
}
