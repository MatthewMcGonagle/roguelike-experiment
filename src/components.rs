const CAPACITY: usize = 10;

#[derive(Clone)]
pub struct EidWithValue<T> {
    pub e_id: Option<u32>,
    pub value: T
}

#[derive(Clone)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

pub struct CoordinateComponents {
    pub values: Vec<EidWithValue<Coordinates>>,
}

impl CoordinateComponents {
    pub fn initialize(capacity: usize) -> CoordinateComponents {
        CoordinateComponents {
            values: Vec::with_capacity(capacity)
        }
    }

    pub fn add(&mut self, e_id: u32, coords: Coordinates) {
        if e_id >= self.values.len().try_into().unwrap() {
            let fill = EidWithValue { e_id: None, value: Coordinates{ x: 0, y: 0 } };
            self.values.resize(e_id.try_into().unwrap(), fill)
        }
        let e_id_conv: usize = e_id.try_into().unwrap();
        self.values[e_id_conv] = EidWithValue { e_id: Some(e_id), value: coords };
    }
}

pub struct Timer { pub time: u32, pub reset: u32 }

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
    pub values: Vec<EidWithValue<Timer>>
}

impl ActionTimers {
    pub fn initialize(capacity: usize) -> ActionTimers {
        ActionTimers {
            values: Vec::with_capacity(CAPACITY)
        }
    }

    pub fn add(&mut self, e_id: u32, timer: Timer) {
        self.values.push(EidWithValue{ e_id: Some(e_id), value: timer });
    }

    pub fn update(&mut self) {
        let ids_of_resets = self.values.iter_mut().map(
            |timer| match timer.value.update() {
                TimerResult::Tick => None,
                TimerResult::Reset => Some(timer.e_id)
            }
        );
        ids_of_resets;
    }
}

pub enum Ai {
    ShiftX
}

pub struct Ais {
    pub values: Vec<EidWithValue<Ai>>
}

impl Ais {
    pub fn initialize(capacity: usize) -> Ais {
        Ais { values: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, e_id: u32, ai: Ai) {
        self.values.push(EidWithValue { e_id: Some(e_id), value: ai });
    }
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
            action_timers: ActionTimers::initialize(CAPACITY),
            ais: Ais::initialize(CAPACITY)
        }
    }
}
