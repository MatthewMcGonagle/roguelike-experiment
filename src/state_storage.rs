use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct StateStorage {
    pub map: String
}
