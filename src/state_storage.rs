use crate::data::*;
use sdl3::pixels::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String,
    pub entities: Vec<EntityStorage>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityStorage {
    pub ai: Option<Ai>,
    pub alignment: Option<AlignmentType>,
    pub blocking: Option<BlockingType>,
    pub coords: Option<Coordinates>,
    pub decision_timer: Option<Timer>,
    pub health: Option<i32>,
    pub render: Option<Render>,
    pub sid: usize,
    pub state: Option<u32>
}
