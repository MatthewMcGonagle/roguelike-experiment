use sdl3::pixels::Color;
use std::slice::Iter;
use std::slice::IterMut;
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

    pub fn add(&mut self, e_id: usize, t: T) {
        let len_needed_for_new = e_id + 1;
        if len_needed_for_new > self.values.len() {
            self.values.resize(len_needed_for_new, None);
        }
        self.values[e_id] = Some(t);
    }

    pub fn get(&self, e_id: usize) -> &Option<T> { & self.values[e_id] }

    pub fn iter_w_eid(&self) -> Enumerate<Iter<'_, Option<T>>> { self.values.iter().enumerate() }

    pub fn iter_mut_w_eid(&mut self) -> Enumerate<IterMut<'_, Option<T>>> { self.values.iter_mut().enumerate() }
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
        self.values.add(e_id, coords)
    }
}

#[derive(Clone)]
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
    pub values: VecIndexedByEid<Timer>
}

impl ActionTimers {
    pub fn initialize(capacity: usize) -> ActionTimers {
        ActionTimers {
            values: VecIndexedByEid::initialize(capacity)
        }
    }

    pub fn add(&mut self, e_id: usize, timer: Timer) {
        self.values.add(e_id, timer);
    }

    fn update_timer(e_id: usize, timer: &mut Timer) -> Option<usize> {
        match timer.update() {
            TimerResult::Tick => None,
            TimerResult::Reset => Some(e_id)
        }
    }

    pub fn update(&mut self) -> Vec<usize> {
        let ids_of_resets = self.values.iter_mut_w_eid().flat_map(
            |(e_id, maybeTimer)| maybeTimer.as_mut().and_then(
                |timer| ActionTimers::update_timer(e_id, timer)
            )
        );
        ids_of_resets.collect()
    }
}

#[derive(Clone)]
pub enum Ai {
    ShiftX,
    AddAvailableSquare 
}

pub struct Ais {
    pub values: VecIndexedByEid<Ai>
}

impl Ais {
    pub fn initialize(capacity: usize) -> Ais {
        Ais { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, e_id: usize, ai: Ai) {
        self.values.add(e_id, ai);
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
        self.values.add(e_id, render)
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
