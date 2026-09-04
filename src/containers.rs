use std::fmt;
use std::iter::Enumerate;
use std::slice::Iter;
use std::slice::IterMut;
use std::collections::HashMap;

use crate::data::Errors;

pub trait ByEid<'a, T> where T: 'a {
    fn get(&self, e_id: usize) -> Option<&T>;
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T>;
    fn remove(&mut self, e_id: usize);
    fn add_or_replace(&mut self, e_id: usize, value: T);
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)>;
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)>;
    fn to_map(&self) -> HashMap<usize, T>;
}

pub struct VecIndexedByEid<T> {
    values: Vec<Option<T>>
}

impl<T: Clone> VecIndexedByEid<T> {
    pub fn initialize(capacity: usize) -> VecIndexedByEid<T> {
        VecIndexedByEid { values: Vec::with_capacity(capacity) }
    }

    pub fn from(values: &HashMap<usize, T>) -> VecIndexedByEid<T> {
        let mut by_eid = VecIndexedByEid::initialize(0);
        for (eid, t) in values {
            by_eid.add_or_replace(*eid, t.clone())
        }

        by_eid
    }

    pub fn from_exactly(maybe_values: &Vec<Option<T>>) -> VecIndexedByEid<T> {
        VecIndexedByEid { values: maybe_values.clone() }
    }

    pub fn add_or_replace(&mut self, e_id: usize, t: T) {
        let len_needed_for_new = e_id + 1;
        if len_needed_for_new > self.values.len() {
            self.values.resize(len_needed_for_new, None);
        }
        self.values[e_id] = Some(t);
    }

    pub fn get(&self, e_id: usize) -> Option<&T> { self.values.get(e_id).map(|x| x.as_ref()).flatten() }

    pub fn get_mut(&mut self, e_id: usize) -> Option<&mut T> { self.values.get_mut(e_id).map(|x| x.as_mut()).flatten() }

    pub fn iter_w_eid(&self) -> Enumerate<Iter<'_, Option<T>>> { self.values.iter().enumerate() }

    pub fn iter_mut_w_eid(&mut self) -> Enumerate<IterMut<'_, Option<T>>> { self.values.iter_mut().enumerate() }

    pub fn remove(&mut self, e_id: usize) { self.values.get_mut(e_id).map(|maybe_x| *maybe_x = None); } 
}

impl<T: PartialEq> PartialEq for VecIndexedByEid<T> {
    fn eq(&self, other: &Self) -> bool { self.values == other.values }
}

impl<T> VecIndexedByEid<Vec<T>> {
    pub fn push(&mut self, e_id: usize, t: T) -> Result<(), Errors> {
        let maybe_current = self.values.get_mut(e_id).ok_or(Errors::MissingExpectedEid)?;
        match maybe_current {
            Some(xs) => Ok(xs.push(t)),
            None => Err(Errors::MissingExpectedEid) 
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for VecIndexedByEid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VecIndexedByEid")
            .field("values", &self.values)
            .finish()
    }
}

pub trait UsesVecIndexedByEid<T> {
    fn the_values(&self) -> &VecIndexedByEid<T>;
    fn mut_values(&mut self) -> &mut VecIndexedByEid<T>;
}

impl<'a, T, U> ByEid<'a, T> for U
where
    T: 'a + Clone,
    U: UsesVecIndexedByEid<T>
{
    fn get(&self, e_id: usize) -> Option<&T> { self.the_values().get(e_id) }
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T> { self.mut_values().get_mut(e_id) }
    fn remove(&mut self, e_id: usize) { self.mut_values().remove(e_id) }
    fn add_or_replace(&mut self, e_id: usize, value: T) {
        self.mut_values().add_or_replace(e_id, value);
    }
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)> { self.the_values().iter_w_eid() }
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)> { self.mut_values().iter_mut_w_eid() }
    fn to_map(&self) -> HashMap<usize, T> {
        HashMap::from_iter(
            self.the_values().values.clone().into_iter()
                .enumerate()
                .flat_map(|(eid, maybe_t)| Some((eid, maybe_t?)))
                .collect::<Vec<(usize, T)>>())
    }
}
