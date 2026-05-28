use crate::data::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String,
    pub entities: Vec<EntityStorage>
}

#[derive(Deserialize, Serialize)]
pub struct EntityStorage {
    pub ai: Option<Ai>,
    pub alignment: Option<AlignmentType>,
    pub blocking: Option<BlockingType>,
    pub coords: Option<Coordinates>,
    pub decision_timer: Option<Timer>,
    pub health: Option<i32>,
    pub render: Option<RenderStorage>,
    pub sid: usize,
    pub state: Option<u32>
}

#[derive(Deserialize, Serialize)]
pub struct RenderStorage {
    pub color: ColorStorage
}

#[derive(Deserialize, Serialize)]
pub struct ColorStorage {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
