use sdl3::pixels::Color;
use std::slice::Iter;
use std::iter::Enumerate;

const CAPACITY: usize = 10;

#[derive(Clone)]
pub struct EidWithValue<T> {
    pub e_id: Option<usize>,
    pub value: T
}

pub struct VecIndexedByEid<T> {
    pub values: Vec<Option<T>>
}

impl<T: Clone> VecIndexedByEid<T> {
    pub fn initialize(capacity: usize) -> VecIndexedByEid<T> {
        VecIndexedByEid { values: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, e_id: usize, t: T, fill: T) {
        let len_needed_for_new = e_id + 1;
        if len_needed_for_new > self.values.len() {
            self.values.resize(len_needed_for_new, None);
        }
        self.values[e_id] = Some(t);
    }

    pub fn get(&self, e_id: usize) -> &Option<T> { & self.values[e_id] }

    pub fn iter_w_eid(&self) -> Enumerate<Iter<'_, Option<T>>> { self.values.iter().enumerate() }
}

fn addForVecIndexedByEid<T: Clone>(values: &mut Vec<EidWithValue<T>>, e_id: usize, t: T, fill: T) {
    let len_needed_for_new = e_id + 1;
    if len_needed_for_new > values.len() {
        let no_e_id_fill = EidWithValue { e_id: None, value: fill };
        values.resize(len_needed_for_new, no_e_id_fill);
    }
    values[e_id] = EidWithValue { e_id: Some(e_id), value: t }
}

#[derive(Clone)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32
}

pub struct CoordinateComponents {
    pub values: VecIndexedByEid<Coordinates>,
}

impl CoordinateComponents {
    pub fn initialize(capacity: usize) -> CoordinateComponents {
        CoordinateComponents {
            values: VecIndexedByEid::initialize(capacity)
        }
    }

    pub fn add(&mut self, e_id: usize, coords: Coordinates) {
        self.values.add(e_id, coords, Coordinates{ x: 0, y: 0})
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

    pub fn add(&mut self, e_id: usize, timer: Timer) {
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

    pub fn add(&mut self, e_id: usize, ai: Ai) {
        self.values.push(EidWithValue { e_id: Some(e_id), value: ai });
    }
}

#[derive(Clone)]
pub struct Render {
    pub color: Color
}

pub struct Renders {
    pub values: VecIndexedByEid<Render>
}

impl Renders {
    pub fn initialize(capacity: usize) -> Renders {
        Renders { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, e_id: usize, render: Render) {
        self.values.add(e_id, render, Render{ color: Color::RGB(0, 0, 0) })
    }

    pub fn get(&self, e_id: usize) -> &Option<Render> { & self.values.get(e_id) }
}

pub struct Components {
    pub coords: CoordinateComponents,
    pub action_timers: ActionTimers,
    pub ais: Ais,
    pub renders: Renders
}

impl Components {
    pub fn initialize() -> Components {
        Components {
            coords: CoordinateComponents::initialize(CAPACITY),
            action_timers: ActionTimers::initialize(CAPACITY),
            ais: Ais::initialize(CAPACITY),
            renders: Renders::initialize(CAPACITY)
        }
    }
}
