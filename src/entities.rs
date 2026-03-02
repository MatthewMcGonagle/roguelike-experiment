use crate::components::*;

pub struct Entities {
    free_ids: Vec<usize>,
    pub active_ids: Vec<usize>
}

const N_IDS: usize = 30;
const ACTIVE_CAPACITY: usize = 30;

impl Entities {
    pub fn initialize() -> Entities {
        let mut the_free_ids: Vec<usize> = (0..N_IDS).collect();
        the_free_ids.reverse();

        Entities {
            free_ids: the_free_ids,
            active_ids: Vec::with_capacity(ACTIVE_CAPACITY)
        }
    }

    pub fn n_free_ids(&self) -> usize {
        self.free_ids.len()
    }

    fn activate_new_id(&mut self) -> Result<usize, Errors> {
        let e_id = self.free_ids.pop().ok_or(Errors::UnexpectedlyEmpty)?;
        self.active_ids.push(e_id);
        Ok(e_id)
    }

    fn free_most_recent_id(&mut self) -> Result<(), Errors> {
        let e_id = self.active_ids.pop().ok_or(Errors::UnexpectedlyEmpty)?;
        self.free_ids.push(e_id);
        Ok(())
    }

    pub fn add_timed_square(
        &mut self, e_components: &mut EntityComponents, coords: Coordinates, time_size: u32, ai: Ai, render: Render
    ) -> Result<usize, Errors> {
        let e_id = self.activate_new_id()?;
        // Make sure we exit if we couldn't add the space data.
        let space_data = match e_components.coords_query.add(coords.x, coords.y, SpaceData::HasEid(e_id)) {
            Err(e) => {
                let _ = self.free_most_recent_id()?;
                Err(e) 
            },
            Ok(x) => Ok(x)
        }?;

        let components = Vec::from([
            space_data,
            e_components.coords.add(e_id, coords),
            e_components.blocking.add(e_id, BlockingType::Movement),
            e_components.action_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            e_components.ais.add(e_id, ai),
            e_components.renders.add(e_id, render)
        ]);
        e_components.component_types.add(e_id, components);
        Ok(e_id)
    }

    pub fn add_timed_square_creator(&mut self, e_components: &mut EntityComponents, coords: Coordinates, time_size: u32) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        let components = Vec::from([
            e_components.coords.add(e_id, coords),
            e_components.action_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            e_components.ais.add(e_id, Ai::AddAvailableSquare),
            e_components.states.add(e_id, 0)
        ]);
        e_components.component_types.add(e_id, components);
        Some(())
    }

    pub fn add_kill_timer(&mut self, e_components: &mut EntityComponents, time_size: u32, target_e_id: usize) -> Result<(), Errors> {
        let e_id = self.free_ids.pop().ok_or(Errors::UnexpectedlyEmpty)?;
        self.active_ids.push(e_id);

        let components = Vec::from([
            e_components.action_timers.add(e_id, Timer { time: time_size, reset: time_size }),
            e_components.ais.add(e_id, Ai::Kill),
            e_components.targets.add(e_id, Vec::from([target_e_id]))
        ]);
        e_components.component_types.add(e_id, components);

        // Need to handle the target too.
        let target_component = e_components.targeted_by.add(target_e_id, Vec::from([e_id]));
        e_components.component_types.push(target_e_id, target_component)
    }

    pub fn remove(&mut self, e_id: usize, e_components: &mut EntityComponents) {
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
        let targeted_by: Vec<usize> = e_components.targeted_by.get(e_id).into_iter().flat_map(|targeted_by| targeted_by.clone()).collect();
        for t_by in targeted_by {
            self.remove(t_by, e_components);
        }

        e_components.component_types.get(e_id).map(
            |c_types| for c_type in c_types { 
                match c_type {
                    ComponentType::ComponentTypeList => (),
                    ComponentType::Coordinates => {
                        e_components.coords.get(e_id).map(|c|
                            e_components.coords_query.get_mut(c.x, c.y).map(|s| *s = SpaceData::Empty)
                        );
                        e_components.coords.remove(e_id);
                    },
                    ComponentType::CoordinatesQuery => (),
                    ComponentType::Blocking => e_components.blocking.remove(e_id),
                    ComponentType::ActionTimer => e_components.action_timers.remove(e_id),
                    ComponentType::Ai => e_components.ais.remove(e_id),
                    ComponentType::State => e_components.states.remove(e_id),
                    ComponentType::Render => e_components.renders.remove(e_id),
                    ComponentType::Target => e_components.targets.remove(e_id),
                    ComponentType::TargetedBy => e_components.targeted_by.remove(e_id)
                }
            }
        );

        e_components.component_types.remove(e_id);
    }
}
