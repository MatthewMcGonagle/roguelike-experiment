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

    pub fn add_timed_square(&mut self, others: &mut OtherComponents, coords: Coordinates, time_size: u32, ai: Ai, render: Render) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        others.coords.add(e_id, coords);
        others.action_timers.add(e_id, Timer { time: time_size, reset: time_size }); 
        others.ais.add(e_id, ai);
        others.renders.add(e_id, render);
        Some(())
    }

    pub fn add_timed_square_creator(&mut self, others: &mut OtherComponents, coords: Coordinates, time_size: u32) -> Option<()> {
        let e_id = self.free_ids.pop()?;
        self.active_ids.push(e_id);

        others.coords.add(e_id, coords);
        others.action_timers.add(e_id, Timer { time: time_size, reset: time_size });
        others.ais.add(e_id, Ai::AddAvailableSquare); 
        Some(())
    }
}
