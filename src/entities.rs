use crate::components::*;

pub struct Entities {
    free_ids: Vec<u32>,
    pub active_ids: Vec<u32>
}

const N_IDS: u32 = 10;
const ACTIVE_CAPACITY: usize = 10;

impl Entities {
    pub fn initialize() -> Entities {
        Entities {
            free_ids: (0..N_IDS).collect(),
            active_ids: Vec::with_capacity(ACTIVE_CAPACITY)
        }
    }

    pub fn add_timed_square(&mut self, components: &mut Components, coords: Coordinates, time_size: u32, ai: Ai) -> Option<()> {
        let id = self.free_ids.pop()?;
        self.active_ids.push(id);

        components.coords.values.push(coords);
        components.action_timers.values.push(Timer { entity: id, time: time_size, reset: time_size }); 
        components.ais.values.push(ai);
        Some(())
    }
}
