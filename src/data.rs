use sdl3::Error;
use sdl3::pixels::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityBuffer {
    pub ai: Option<Ai>,
    pub alignment: Option<AlignmentType>,
    pub blocking: Option<BlockingType>,
    pub coords: Option<Coordinates>,
    pub decision_timer: Option<Timer>,
    pub health: Option<i32>,
    pub render: Option<Render>,
    pub state: Option<u32>
}

#[derive(Debug)]
pub enum Errors {
    UnknownWorldState(String),
    CoordinateMissing,
    MissingExpectedEid,
    SpaceAlreadyNonempty,
    UnexpectedlyEmpty,
    NotExpectingAiForUser,
    SDL3Error(Error)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize 
}

#[derive(Clone, Debug)]
pub enum WorldState {
    Spawner(usize, usize, u32),
    Wall(usize, usize)
}

#[derive(Clone, PartialEq)]
pub enum SpaceData {
    Empty,
    HasEid(usize)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum BlockingType {
    Movement
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Ai {
    AlternateDirections(usize, Direction, Direction),
    AddAvailableSquare,
    Kill,
    User
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ColorBuffer {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl ColorBuffer {
    pub fn RGB(r: u8, g: u8, b: u8) -> ColorBuffer {
        ColorBuffer { r: r, g: g, b: b, a: 255 }
    }

    pub fn to_color(&self) -> Color {
        Color { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Render {
    pub color: ColorBuffer
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
