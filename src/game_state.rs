use crate::components::*;
use crate::components::for_entities::*;
use crate::data::*;
use crate::entities::*;

const CAPACITY: usize = 10;

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

pub struct GameState {
    pub loop_state: LoopState,
    pub display: Display,
    pub decisions_ready: DecisionsReady,
    pub planned_actions: PlannedActions,
    pub reactions_ready: ReactionsReady,
    pub to_kill: ToKill,
    pub components: Components,
    pub entities: Entities
}

impl GameState {
    pub fn initialize(loop_state: LoopState, display: Display, coord_width: usize, coord_height: usize) -> GameState {
        GameState {
            loop_state: loop_state,
            display: display,
            decisions_ready: DecisionsReady::initialize(CAPACITY),
            planned_actions: PlannedActions::initialize(CAPACITY),
            reactions_ready: ReactionsReady::initialize(CAPACITY),
            to_kill: ToKill::initialize(CAPACITY),
            components: Components::initialize(CAPACITY, coord_width, coord_height),
            entities: Entities::initialize()
        }
    }
}
