use crate::components::*;

pub struct Entities {
    free_ids: Vec<usize>,
    pub active_ids: Vec<usize>
}

const N_IDS: usize = 10;
const ACTIVE_CAPACITY: usize = 10;

impl Entities {
    pub fn initialize() -> Entities {
        Entities {
            free_ids: (0..N_IDS).collect(),
            active_ids: Vec::with_capacity(ACTIVE_CAPACITY)
        }
    }

    pub fn add_timed_square(&mut self, e_components: &mut EntityComponents, coords: Coordinates, time_size: u32, ai: Ai, render: Render) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        e_components.coords.add(&mut e_components.component_types, e_id, coords);
        e_components.action_timers.add(&mut e_components.component_types, e_id, Timer { time: time_size, reset: time_size }); 
        e_components.ais.add(&mut e_components.component_types, e_id, ai);
        e_components.renders.add(&mut e_components.component_types, e_id, render);
        Some(())
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
}
