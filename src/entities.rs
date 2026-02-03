pub struct Entities {
    free_ids: Vec<u32>,
    pub active_ids: Vec<u32>
}

const N_IDS: u32 = 10;

impl Entities {

    pub fn initialize() -> Entities {
        Entities {
            free_ids: (0..N_IDS).collect(),
            active_ids: Vec::with_capacity(N_IDS.try_into().unwrap())
        }
    }
}
