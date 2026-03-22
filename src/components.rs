mod containers;
pub mod for_entities;

use containers::*;
use crate::data::*;
use for_entities::*;

const CAPACITY: usize = 10;

pub trait Component<'a, T> where T: 'a {
    fn get(&self, e_id: usize) -> Option<&T>;
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T>;
    fn add(&mut self, e_id: usize, value: T) -> ComponentType;
    fn remove(&mut self, e_id: usize);
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)>;
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)>;
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

pub struct PlannedActions {
    pub values: Vec<Action>
}

impl PlannedActions {
    pub fn initialize(capacity: usize) -> PlannedActions {
        PlannedActions { values: Vec::with_capacity(capacity) }
    }
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
