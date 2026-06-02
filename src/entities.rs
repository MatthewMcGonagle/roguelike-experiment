mod free_eids;

use crate::components::*;
use crate::data::*;
use crate::state_storage::*;
use free_eids::FreeEids;

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

    pub fn add_space_data_or_free_recent_eid(
        &mut self, components: &mut Components, coords: &Coordinates, space_data: SpaceData) -> Result<ComponentType, Errors>
    {
        // Make sure we exit if we couldn't add the space data.
        match components.coords_query.add(coords.x, coords.y, space_data) {
            Err(e) => {
                let _ = self.free_most_recent_id()?;
                Err(e)
            },
            Ok(x) => Ok(x)
        }
    }

    pub fn add_entity_storage(&mut self, components: &mut Components, entity_storage: EntityStorage) -> Result<(), Errors> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        let maybe_space_component = entity_storage.coords.as_ref().map(|c|
            self.add_space_data_or_free_recent_eid(components, c, SpaceData::HasEid(e_id)))
            .transpose()?;

        let components_added = Vec::from([
            entity_storage.ai.map(|ai| components.ais.add(e_id, ai)),
            entity_storage.alignment.map(|a| components.alignments.add(e_id, a)),
            entity_storage.blocking.map(|b| components.blocking.add(e_id, b)),
            entity_storage.coords.map(|cs| components.coords.add(e_id, cs)),
            maybe_space_component,
            entity_storage.decision_timer.map(|dt| components.decision_timers.add(e_id, dt)),
            entity_storage.health.map(|h| components.healths.add(e_id, h)),
            entity_storage.render.map(|r| components.renders.add(e_id, r.to_render())),
            entity_storage.state.map(|s| components.states.add(e_id, s))
        ]).into_iter().flatten().collect();
        components.component_types.add(e_id, components_added);

        Ok(())
    }

    pub fn add_wall_block(&mut self, components: &mut Components, coords: Coordinates, render: Render) -> Result<usize, Errors> {
        let e_id = self.free_ids.pop()?;
        // Make sure we exit if we couldn't add the space data.
        let space_data = match components.coords_query.add(coords.x, coords.y, SpaceData::HasEid(e_id)) {
            Err(e) => {
                let _ = self.free_most_recent_id()?;
                Err(e) 
            },
            Ok(x) => Ok(x)
        }?;

        let components_added = Vec::from([
            space_data,
            components.coords.add(e_id, coords),
            components.blocking.add(e_id, BlockingType::Movement),
            components.renders.add(e_id, render)
        ]);
        components.component_types.add(e_id, components_added);
        Ok(e_id)
    }

    pub fn add_timed_square(
        &mut self, components: &mut Components, coords: Coordinates, time_size: u32, ai: Ai, alignment: AlignmentType, health: i32, render: Render
    ) -> Result<usize, Errors> {
        let e_id = self.free_ids.pop()?;
        // Make sure we exit if we couldn't add the space data.
        let space_data = match components.coords_query.add(coords.x, coords.y, SpaceData::HasEid(e_id)) {
            Err(e) => {
                let _ = self.free_most_recent_id()?;
                Err(e) 
            },
            Ok(x) => Ok(x)
        }?;

        let components_added = Vec::from([
            space_data,
            components.coords.add(e_id, coords),
            components.blocking.add(e_id, BlockingType::Movement),
            components.decision_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            components.ais.add(e_id, ai),
            components.alignments.add(e_id, alignment),
            components.healths.add(e_id, health),
            components.renders.add(e_id, render)
        ]);
        components.component_types.add(e_id, components_added);
        Ok(e_id)
    }

    pub fn add_timed_square_creator(&mut self, components: &mut Components, coords: Coordinates, time_size: u32) -> Result<(), Errors> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        let components_added = Vec::from([
            components.coords.add(e_id, coords),
            components.decision_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            components.ais.add(e_id, Ai::AddAvailableSquare),
            components.states.add(e_id, 0)
        ]);
        components.component_types.add(e_id, components_added);
        Ok(())
    }

    pub fn add_kill_timer(&mut self, components: &mut Components, time_size: u32, target_e_id: usize) -> Result<(), Errors> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        let components_added = Vec::from([
            components.decision_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            components.ais.add(e_id, Ai::Kill),
            components.targets.add(e_id, Vec::from([target_e_id]))
        ]);
        components.component_types.add(e_id, components_added);

        // Need to handle the target too.
        let target_component = components.targeted_by.add(target_e_id, Vec::from([e_id]));
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
        let targeted_by: Vec<usize> = components.targeted_by.get(e_id).into_iter().flat_map(|targeted_by| targeted_by.clone()).collect();
        for t_by in targeted_by {
            self.remove(t_by, components);
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
                    ComponentType::Target => components.targets.remove(e_id),
                    ComponentType::TargetedBy => components.targeted_by.remove(e_id),
                    ComponentType::Alignment => components.alignments.remove(e_id),
                    ComponentType::Health => components.healths.remove(e_id)
                }
            }
        );

        components.component_types.remove(e_id);
    }
}
