use crate::components::*;

pub struct Entities {
    free_ids: Vec<usize>,
    pub active_ids: Vec<usize>
}

const N_IDS: usize = 30;
const ACTIVE_CAPACITY: usize = 30;

impl Entities {
    pub fn initialize() -> Entities {
        Entities {
            free_ids: (0..N_IDS).collect(),
            active_ids: Vec::with_capacity(ACTIVE_CAPACITY)
        }
    }

    pub fn n_free_ids(&self) -> usize {
        self.free_ids.len()
    }

    pub fn add_timed_square(&mut self, e_components: &mut EntityComponents, coords: Coordinates, time_size: u32, ai: Ai, render: Render) -> Option<usize> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        e_components.coords.add(&mut e_components.component_types, e_id, coords);
        e_components.blocking.add(&mut e_components.component_types, e_id);
        e_components.action_timers.add(&mut e_components.component_types, e_id, Timer { time: time_size, reset: time_size }); 
        e_components.ais.add(&mut e_components.component_types, e_id, ai);
        e_components.renders.add(&mut e_components.component_types, e_id, render);
        Some(e_id)
    }

    pub fn add_timed_square_creator(&mut self, e_components: &mut EntityComponents, coords: Coordinates, time_size: u32) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        e_components.coords.add(&mut e_components.component_types, e_id, coords);
        e_components.action_timers.add(&mut e_components.component_types, e_id, Timer { time: time_size, reset: time_size });
        e_components.ais.add(&mut e_components.component_types, e_id, Ai::AddAvailableSquare); 
        e_components.states.add(&mut e_components.component_types, e_id, 0);
        Some(())
    }

    pub fn add_kill_timer(&mut self, e_components: &mut EntityComponents, time_size: u32, target_e_id: usize) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        e_components.action_timers.add(&mut e_components.component_types, e_id, Timer { time: time_size, reset: time_size });
        e_components.ais.add(&mut e_components.component_types, e_id, Ai::Kill);
        e_components.targets.add(&mut e_components.component_types, e_id, target_e_id);
        e_components.targeted_by.add(&mut e_components.component_types, target_e_id, e_id);
        Some(())
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
        let targeted_by: Vec<usize> = e_components.targeted_by.values.get(e_id).into_iter().flat_map(|targeted_by| targeted_by.clone()).collect();
        for t_by in targeted_by {
            self.remove(t_by, e_components);
        }

        e_components.component_types.values.get(e_id).map(
            |c_types| for c_type in c_types { 
                match c_type {
                    ComponentType::Coordinates => {
                        e_components.coords.values.get(e_id).map(|c|
                            e_components.coords_query.get_mut(c.x, c.y).map(|s| *s = SpaceData::Empty)
                        );
                        e_components.coords.values.remove(e_id);
                    },
                    ComponentType::Blocking => e_components.blocking.values.remove(e_id),
                    ComponentType::ActionTimer => e_components.action_timers.values.remove(e_id),
                    ComponentType::Ai => e_components.ais.values.remove(e_id),
                    ComponentType::State => e_components.states.values.remove(e_id),
                    ComponentType::Render => e_components.renders.values.remove(e_id),
                    ComponentType::Target => e_components.targets.values.remove(e_id),
                    ComponentType::TargetedBy => e_components.targeted_by.values.remove(e_id)
                }
            }
        );

        e_components.component_types.values.remove(e_id);
    }
}
