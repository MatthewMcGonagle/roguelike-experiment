use sdl3::pixels::Color;

pub enum Errors {
    CoordinateMissing,
    MissingExpectedEid,
    SpaceAlreadyNonempty,
    UnexpectedlyEmpty,
    NotExpectingAiForUser
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

pub enum Reaction {
    Kill(usize)
}
