use crate::data::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String,
    pub entities: Vec<EntityStorage>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityStorage {
    pub sid: usize,
    pub entity: EntityBuffer
}
