use sdl3::pixels::Color;
use std::slice::Iter;
use std::slice::IterMut;
use std::iter::Enumerate;

const CAPACITY: usize = 10;

pub enum Errors {
    CoordinateMissing,
    MissingExpectedEid,
    SpaceAlreadyNonempty,
    UnexpectedlyEmpty
}

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

pub trait Component<'a, T> where T: 'a {
    fn get(&self, e_id: usize) -> Option<&T>;
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T>;
    fn add(&mut self, e_id: usize, value: T) -> ComponentType;
    fn remove(&mut self, e_id: usize);
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &Option<T>)>;
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &mut Option<T>)>;
}

trait UsesVecIndexedByEid<T> {
    fn the_values(&self) -> &VecIndexedByEid<T>;
    fn mut_values(&mut self) -> &mut VecIndexedByEid<T>;
    fn component_type() -> ComponentType;
}

impl<'a, T, U> Component<'a, T> for U
where
    T: 'a + Clone,
    U: UsesVecIndexedByEid<T>
{
    fn get(&self, e_id: usize) -> Option<&T> { self.the_values().get(e_id) }
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T> { self.mut_values().get_mut(e_id) }
    fn add(&mut self, e_id: usize, value: T) -> ComponentType {
        self.mut_values().add(e_id, value);
        U::component_type()
    }
    fn remove(&mut self, e_id: usize) { self.mut_values().remove(e_id) }
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &Option<T>)> { self.the_values().iter_w_eid() }
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &mut Option<T>)> { self.mut_values().iter_mut_w_eid() }
}

#[derive(Clone)]
pub enum ComponentType {
    Coordinates,
    CoordinatesQuery,
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

    pub fn add(&mut self, e_id: usize, c_types: Vec<ComponentType>) {
        self.values.add(e_id, c_types);
    }

    pub fn push(&mut self, e_id: usize, c_type: ComponentType) -> Result<(), Errors> {
        let current = self.values.get_mut(e_id).ok_or(Errors::MissingExpectedEid)?;
        current.push(c_type);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize 
}

pub struct CoordinateComponents {
    values: VecIndexedByEid<Coordinates>,
}

impl CoordinateComponents {
    pub fn initialize(capacity: usize) -> CoordinateComponents {
        CoordinateComponents {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<Coordinates> for CoordinateComponents {
    fn the_values(&self) -> &VecIndexedByEid<Coordinates> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Coordinates> { &mut self.values }
    fn component_type() -> ComponentType { ComponentType::Coordinates}
}

#[derive(Clone)]
pub enum SpaceData {
    Empty,
    HasEid(usize)
}

pub struct CoordinatesQuery {
    pub coord_width: usize,
    pub coord_height: usize,
    values: Vec<SpaceData>
}

impl CoordinatesQuery {
    pub fn initialize(coord_width: usize, coord_height: usize) -> CoordinatesQuery {
        let mut the_values: Vec<SpaceData> = Vec::with_capacity(coord_width * coord_height);
        the_values.resize(coord_width * coord_height, SpaceData::Empty);

        CoordinatesQuery {
            coord_width: coord_width,
            coord_height: coord_height,
            values: the_values 
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&SpaceData, Errors> {
        self.values.get(y * self.coord_width + x).ok_or(Errors::CoordinateMissing)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut SpaceData, Errors> {
        self.values.get_mut(y * self.coord_width + x).ok_or(Errors::CoordinateMissing)
    }

    pub fn add(&mut self, x: usize, y: usize, space_data: SpaceData) -> Result<ComponentType, Errors> {
        let space = self.get_mut(x, y)?;
        match space {
            SpaceData::Empty => {
                *space = space_data;
                Ok(ComponentType::CoordinatesQuery)
            },
            _ => Err(Errors::SpaceAlreadyNonempty) 
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum BlockingType {
    Movement
}

pub struct Blocking {
    values: VecIndexedByEid<BlockingType>
}

impl Blocking {
    pub fn initialize(capacity: usize) -> Blocking {
        Blocking {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<BlockingType> for Blocking {
    fn the_values(&self) -> &VecIndexedByEid<BlockingType> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<BlockingType> { &mut self.values }
    fn component_type() -> ComponentType { ComponentType::Blocking }
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
    values: VecIndexedByEid<Timer>
}

impl ActionTimers {
    pub fn initialize(capacity: usize) -> ActionTimers {
        ActionTimers {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<Timer> for ActionTimers {
    fn the_values(&self) -> &VecIndexedByEid<Timer> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Timer> { &mut self.values }
    fn component_type() -> ComponentType { ComponentType::ActionTimer }
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

    pub fn add(&mut self, e_id: usize, ai: Ai) -> ComponentType {
        self.values.add(e_id, ai);
        ComponentType::Ai
    }
}

pub struct States {
    pub values: VecIndexedByEid<u32>
}

impl States {
    pub fn initialize(capacity: usize) -> States {
        States { values: VecIndexedByEid::initialize(capacity) }
    }

    pub fn add(&mut self, e_id: usize, state: u32) -> ComponentType {
        self.values.add(e_id, state);
        ComponentType::State
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

    pub fn add(&mut self, e_id: usize, render: Render) -> ComponentType {
        self.values.add(e_id, render);
        ComponentType::Render
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

    pub fn add(&mut self, e_id: usize, target_e_id: usize) -> ComponentType {
        match self.values.get_mut(e_id) {
            None => self.values.add(e_id, Vec::new()),
            _ => ()
        };
        self.values.get_mut(e_id).map(|targets| targets.push(target_e_id));
        ComponentType::Target
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

    pub fn add(&mut self, e_id: usize, targeted_by_e_id: usize) -> ComponentType {
        // TODO: this could double up on TargetedBy if an entity is targeted by more than one other
        // entity. Is this a problem?
        match self.values.get_mut(e_id) {
            None => self.values.add(e_id, Vec::new()),
            _ => ()
        };
        self.values.get_mut(e_id).map(|targets| targets.push(targeted_by_e_id));
        ComponentType::TargetedBy
    }
}

pub struct EntityComponents {
    pub component_types: ComponentTypes,
    pub coords: CoordinateComponents,
    pub coords_query: CoordinatesQuery,
    pub blocking: Blocking,
    pub action_timers: ActionTimers,
    pub ais: Ais,
    pub states: States,
    pub renders: Renders,
    pub targets: Targets,
    pub targeted_by: TargetedBy
}

impl EntityComponents {
    pub fn initialize(capacity: usize, coord_width: usize, coord_height: usize) -> EntityComponents {
        EntityComponents {
            component_types: ComponentTypes::initialize(capacity),
            coords: CoordinateComponents::initialize(capacity),
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height),
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
    pub coord_scale: usize
}

pub struct Components {
    pub display: Display,
    pub actions_ready: ActionsReady,
    pub e_components: EntityComponents
}

impl Components {
    pub fn initialize(display: Display, coord_width: usize, coord_height: usize) -> Components {
        Components {
            display: display,
            actions_ready: ActionsReady::initialize(CAPACITY),
            e_components: EntityComponents::initialize(CAPACITY, coord_width, coord_height)
        }
    }
}
