use crate::data::*;

pub struct FreeEids {
    free_ids_allocation_size: usize,
    exclusive_max_eid: usize,
    eids: Vec<usize>
}

impl FreeEids {
    pub fn initialize(free_ids_allocation_size: usize) -> FreeEids {
        let mut the_free_ids: Vec<usize> = (0..free_ids_allocation_size).collect();
        the_free_ids.reverse();

        FreeEids {
            free_ids_allocation_size: free_ids_allocation_size,
            exclusive_max_eid: free_ids_allocation_size,
            eids: the_free_ids
        }
    }

    fn allocate_new_ids(&mut self) {
        let new_max_eid = self.exclusive_max_eid + self.free_ids_allocation_size;
        let mut new_eids: Vec<usize> = (self.exclusive_max_eid..new_max_eid).collect();
        new_eids.reverse();

        for id in new_eids {
            self.eids.push(id);
        }
        self.exclusive_max_eid = new_max_eid;
    }

    pub fn n_free_ids(&self) -> usize {
        self.eids.len()
    }

    pub fn pop(&mut self) -> Result<usize, Errors> {
        if self.n_free_ids() == 0 {
            self.allocate_new_ids();
        }

        let e_id = self.eids.pop().ok_or(Errors::UnexpectedlyEmpty)?;
        Ok(e_id)
    }

    pub fn push(&mut self, eid: usize) {
        self.eids.push(eid);
    }
}
