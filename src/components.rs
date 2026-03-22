pub mod for_entities;

use for_entities::*;
use sdl3::pixels::Color;
use std::slice::Iter;
use std::slice::IterMut;
use std::iter::Enumerate;

const CAPACITY: usize = 10;

pub enum Errors {
    CoordinateMissing,
    MissingExpectedEid,
    SpaceAlreadyNonempty,
    UnexpectedlyEmpty,
    NotExpectingAiForUser
}

pub struct VecIndexedByEid<T> {
    values: Vec<Option<T>>
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
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)>;
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)>;
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
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)> { self.the_values().iter_w_eid() }
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)> { self.mut_values().iter_mut_w_eid() }
}

#[derive(Clone)]
pub enum ComponentType {
    ComponentTypeList,
    Coordinates,
    CoordinatesQuery,
    DecisionTimer,
    Ai,
    State,
    Render,
    Target,
    TargetedBy,
    Blocking,
    Alignment,
    Health
}

#[derive(Clone)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize 
}

#[derive(Clone)]
pub enum SpaceData {
    Empty,
    HasEid(usize)
}

#[derive(Clone, PartialEq)]
pub enum BlockingType {
    Movement
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

#[derive(Clone)]
pub enum Ai {
    ShiftX,
    ShiftY,
    AddAvailableSquare,
    Kill,
    User
}

#[derive(Clone)]
pub struct Render {
    pub color: Color
}

#[derive(Clone)]
pub enum AlignmentType {
    User,
    Neutral,
    HostileToUser
}

pub struct EntityComponents {
    pub component_types: ComponentTypes,
    pub coords: CoordinateComponents,
    pub coords_query: CoordinatesQuery,
    pub blocking: Blocking,
    pub decision_timers: DecisionTimers,
    pub ais: Ais,
    pub states: States,
    pub renders: Renders,
    pub targets: Targets,
    pub targeted_by: TargetedBy,
    pub alignments: Alignments,
    pub healths: Healths
}

impl EntityComponents {
    pub fn initialize(capacity: usize, coord_width: usize, coord_height: usize) -> EntityComponents {
        EntityComponents {
            component_types: ComponentTypes::initialize(capacity),
            coords: CoordinateComponents::initialize(capacity),
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height),
            blocking: Blocking::initialize(capacity),
            decision_timers: DecisionTimers::initialize(capacity),
            ais: Ais::initialize(capacity),
            states: States::initialize(capacity),
            renders: Renders::initialize(capacity),
            targets: Targets::initialize(capacity),
            targeted_by: TargetedBy::initialize(capacity),
            alignments: Alignments::initialize(capacity),
            healths: Healths::initialize(capacity)
        }
    }
}

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub coord_scale: usize
}

#[derive(PartialEq)]
pub enum LoopState {
    RunTimers,
    MakeDecisions,
    DoActions,
    User(usize)
}

pub enum Direction {
    Down,
    Up,
    Right,
    Left
}

pub enum Action {
    Move(usize, Direction),
    Attack(usize, usize),
    Spawn(usize),
    Kill(usize),
    Wait
}

pub struct PlannedActions {
    pub values: Vec<Action>
}

impl PlannedActions {
    pub fn initialize(capacity: usize) -> PlannedActions {
        PlannedActions { values: Vec::with_capacity(capacity) }
    }
}

pub enum Reaction {
    Kill(usize)
}

pub struct ReactionsReady {
    pub values: Vec<Reaction>
}

impl ReactionsReady {
    pub fn initialize(capacity: usize) -> ReactionsReady {
        ReactionsReady { values: Vec::with_capacity(capacity) }
    }
}

pub struct ToKill {
    pub values: Vec<usize>
}

impl ToKill {
    pub fn initialize(capacity: usize) -> ToKill {
        ToKill { values: Vec::with_capacity(capacity) }
    }
}

pub struct Components {
    pub loop_state: LoopState,
    pub display: Display,
    pub decisions_ready: DecisionsReady,
    pub planned_actions: PlannedActions,
    pub reactions_ready: ReactionsReady,
    pub to_kill: ToKill,
    pub e_components: EntityComponents
}

impl Components {
    pub fn initialize(loop_state: LoopState, display: Display, coord_width: usize, coord_height: usize) -> Components {
        Components {
            loop_state: loop_state,
            display: display,
            decisions_ready: DecisionsReady::initialize(CAPACITY),
            planned_actions: PlannedActions::initialize(CAPACITY),
            reactions_ready: ReactionsReady::initialize(CAPACITY),
            to_kill: ToKill::initialize(CAPACITY),
            e_components: EntityComponents::initialize(CAPACITY, coord_width, coord_height)
        }
    }
}
