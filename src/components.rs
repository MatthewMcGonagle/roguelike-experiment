const CAPACITY: usize = 10;

pub struct EidWithValue<T> {
    pub e_id: u32,
    pub value: T
}

pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

pub struct CoordinateComponents {
    pub values: Vec<EidWithValue<Coordinates>>,
    pub e_id_indices: Vec<usize>
}

impl CoordinateComponents {
    pub fn initialize(capacity: usize) -> CoordinateComponents {
        CoordinateComponents {
            values: Vec::with_capacity(capacity),
            e_id_indices: Vec::with_capacity(capacity)
        }
    }

    pub fn add(&mut self, e_id: u32, coords: Coordinates) {
        self.values.push(EidWithValue { e_id: e_id, value: coords });
        self.e_id_indices.push(self.values.len() - 1)
    }
}

pub struct Timer { pub entity: u32, pub time: u32, pub reset: u32 }

enum TimerResult {
    Tick,
    Reset
}

impl Timer {
    fn update(&mut self) -> TimerResult {
        self.time = self.time - 1;
        if self.time <= 0 {
            self.time = self.reset;
            return TimerResult::Reset;
        }
        return TimerResult::Tick;
    }
}

pub struct ActionTimers {
    pub values: Vec<Timer>
}

impl ActionTimers {
    pub fn update(&mut self) {
        let ids_of_resets = self.values.iter_mut().map(
            |timer| match timer.update() {
                TimerResult::Tick => None,
                TimerResult::Reset => Some(timer.entity)
            }
        );
        ids_of_resets;
    }
}

pub enum Ai {
    ShiftX
}

pub struct Ais {
    pub values: Vec<Ai>
}

pub struct Components {
    pub coords: CoordinateComponents,
    pub action_timers: ActionTimers,
    pub ais: Ais
}

impl Components {
    pub fn initialize() -> Components {
        Components {
            coords: CoordinateComponents::initialize(CAPACITY),
            action_timers: ActionTimers { 
                values: Vec::with_capacity(CAPACITY)
            },
            ais: Ais {
                values: Vec::with_capacity(CAPACITY)
            }
        }
    }
}
