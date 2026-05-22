use crate::data::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String,
    pub entities: Vec<EntityStorage>
}

#[derive(Deserialize, Serialize)]
pub struct EntityStorage {
    pub sid: usize,
    pub coords: Coordinates
}
