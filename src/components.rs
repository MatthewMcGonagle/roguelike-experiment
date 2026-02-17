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

    pub fn get(&self, e_id: usize) -> Option<&T> { self.values.get(e_id).map(|x| x.as_ref()).flatten() }

    pub fn get_mut(&mut self, e_id: usize) -> Option<&mut T> { self.values.get_mut(e_id).map(|x| x.as_mut()).flatten() }

    pub fn iter_w_eid(&self) -> Enumerate<Iter<'_, Option<T>>> { self.values.iter().enumerate() }

    pub fn iter_mut_w_eid(&mut self) -> Enumerate<IterMut<'_, Option<T>>> { self.values.iter_mut().enumerate() }

    pub fn remove(&mut self, e_id: usize) { self.values.get_mut(e_id).map(|maybe_x| *maybe_x = None); } 
}

#[derive(Clone)]
pub enum ComponentType {
    Coordinates,
    ActionTimer,
    Ai,
    State,
    Render,
    Target,
    TargetedBy,
    Blocking
}

pub struct ComponentTypes {
    pub values: VecIndexedByEid<Vec<ComponentType>>
}

impl ComponentTypes {
    pub fn initialize(e_id_capacity: usize) -> ComponentTypes {
        ComponentTypes { values: VecIndexedByEid::initialize(e_id_capacity) }
    }

    const CT_CAPACITY: usize = 10;
    pub fn add(&mut self, e_id: usize, c_type: ComponentType) {
        let maybe_types = self.values.get_mut(e_id);
        let types: Option<&mut Vec<ComponentType>> = match maybe_types {
            None => {
                self.values.add(e_id, Vec::with_capacity(ComponentTypes::CT_CAPACITY));
                self.values.get_mut(e_id)
            },
            _ => maybe_types
        };
        types.map(|ts| ts.push(c_type));
    }
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

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, coords: Coordinates) {
        component_types.add(e_id, ComponentType::Coordinates);
        self.values.add(e_id, coords)
    }
}

pub struct Blocking {
    pub values: VecIndexedByEid<bool>
}

impl Blocking {
    pub fn initialize(capacity: usize) -> Blocking {
        Blocking {
            values: VecIndexedByEid::initialize(capacity)
        }
    }

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize) {
        component_types.add(e_id, ComponentType::Blocking);
        self.values.add(e_id, true);
    }
}

#[derive(Clone)]
pub struct Timer { pub time: u32, pub reset: u32 }

pub enum TimerResult {
    Tick,
    Reset
}

impl Timer {
    pub fn update(&mut self) -> TimerResult {
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

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, timer: Timer) {
        component_types.add(e_id, ComponentType::ActionTimer);
        self.values.add(e_id, timer);
    }
}

#[derive(Clone)]
pub enum Ai {
    ShiftX,
    ShiftY,
    AddAvailableSquare,
    Kill
}

pub struct Ais {
    pub values: VecIndexedByEid<Ai>
}

impl Ais {
    pub fn initialize(capacity: usize) -> Ais {
        Ais { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, ai: Ai) {
        component_types.add(e_id, ComponentType::Ai);
        self.values.add(e_id, ai);
    }
}

pub struct States {
    pub values: VecIndexedByEid<u32>
}

impl States {
    pub fn initialize(capacity: usize) -> States {
        States { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, c_types: &mut ComponentTypes, e_id: usize, state: u32) {
        c_types.add(e_id, ComponentType::State);
        self.values.add(e_id, state);
    }
}

pub struct ActionsReady {
    pub values: Vec<usize>
}

impl ActionsReady {
    pub fn initialize(capacity: usize) -> ActionsReady {
        ActionsReady { values: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, e_id: usize) { self.values.push(e_id) }
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

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, render: Render) {
        component_types.add(e_id, ComponentType::Render);
        self.values.add(e_id, render)
    }

    pub fn get(&self, e_id: usize) -> Option<&Render> { self.values.get(e_id) }
}

pub struct Targets {
    pub values: VecIndexedByEid<Vec<usize>>
}

impl Targets {
    pub fn initialize(capacity: usize) -> Targets {
        Targets { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, target_e_id: usize) {
        component_types.add(e_id, ComponentType::Target);
        match self.values.get_mut(e_id) {
            None => self.values.add(e_id, Vec::new()),
            _ => ()
        };
        self.values.get_mut(e_id).map(|targets| targets.push(target_e_id));
    }
}

// If we kill this e_id then we need to appropriately updates other entities that target this one.
pub struct TargetedBy {
    pub values: VecIndexedByEid<Vec<usize>>
}

impl TargetedBy {
    pub fn initialize(capacity: usize) -> TargetedBy {
        TargetedBy { values: VecIndexedByEid::initialize(capacity) } 
    }

    pub fn add(&mut self, component_types: &mut ComponentTypes, e_id: usize, targeted_by_e_id: usize) {
        // TODO: this could double up on TargetedBy if an entity is targeted by more than one other
        // entity. Is this a problem?
        component_types.add(e_id, ComponentType::TargetedBy);
        match self.values.get_mut(e_id) {
            None => self.values.add(e_id, Vec::new()),
            _ => ()
        };
        self.values.get_mut(e_id).map(|targets| targets.push(targeted_by_e_id));
    }
}

pub struct EntityComponents {
    pub component_types: ComponentTypes,
    pub coords: CoordinateComponents,
    pub blocking: Blocking,
    pub action_timers: ActionTimers,
    pub ais: Ais,
    pub states: States,
    pub renders: Renders,
    pub targets: Targets,
    pub targeted_by: TargetedBy
}

impl EntityComponents {
    pub fn initialize(capacity: usize) -> EntityComponents {
        EntityComponents {
            component_types: ComponentTypes::initialize(capacity),
            coords: CoordinateComponents::initialize(capacity),
            blocking: Blocking::initialize(capacity),
            action_timers: ActionTimers::initialize(capacity),
            ais: Ais::initialize(capacity),
            states: States::initialize(capacity),
            renders: Renders::initialize(capacity),
            targets: Targets::initialize(capacity),
            targeted_by: TargetedBy::initialize(capacity)
        }
    }
}

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub coord_scale: u32
}

impl Display {
    pub fn coord_width(&self) -> u32 { self.width / self.coord_scale }
    pub fn coord_height(&self) -> u32 { self.height / self.coord_scale }
}

pub struct Components {
    pub display: Display,
    pub actions_ready: ActionsReady,
    pub e_components: EntityComponents
}

impl Components {
    pub fn initialize(display: Display) -> Components {
        Components {
            display: display,
            actions_ready: ActionsReady::initialize(CAPACITY),
            e_components: EntityComponents::initialize(CAPACITY)
        }
    }
}
