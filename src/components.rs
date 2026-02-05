const CAPACITY: usize = 10;

pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct CoordinateComponents {
    pub values: Vec<Coordinates>,
}

impl CoordinateComponents {
    pub fn add(&mut self, coords: Coordinates) {
        self.values.push(coords)
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
        for timer in self.values.iter_mut() {
            match timer.update() {
                TimerResult::Tick => None,
                TimerResult::Reset => Some(timer.entity)
            };
        }
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
            coords: CoordinateComponents {
                values: Vec::with_capacity(CAPACITY)
            },
            action_timers: ActionTimers { 
                values: Vec::with_capacity(CAPACITY)
            },
            ais: Ais {
                values: Vec::with_capacity(CAPACITY)
            }
        }
    }
}
