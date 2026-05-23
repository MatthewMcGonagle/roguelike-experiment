use crate::data::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String,
    pub entities: Vec<EntityStorage>
}

#[derive(Deserialize, Serialize)]
pub struct EntityStorage {
    // Fields in order to define in toml; flat fields first.
    pub sid: usize,
    pub state: Option<u32>,
    pub ai: Option<Ai>,
    pub blocking: Option<BlockingType>,
    pub coords: Option<Coordinates>,
    pub decision_timer: Option<Timer>
}
