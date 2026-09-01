pub mod for_entities;

use crate::containers::*;
use crate::data::*;
use for_entities::*;
use std::collections::HashMap;

pub trait ComponentData<'a, T, U> where
    T: 'a + Clone,
    U: ByEid<'a, T>
{
    fn by_eid(&self) -> &U;
    fn mut_by_eid(&mut self) -> &mut U;
    fn component_type() -> ComponentType;

    fn add(&mut self, e_id: usize, value: T) -> ComponentType {
        self.mut_by_eid().add_or_replace(e_id, value);
        Self::component_type()
    }
}

pub trait AssociatedComponentType {
    fn associated() -> ComponentType;
}

impl<'a, T, U> ComponentData<'a, T, U> for U
where
    T: 'a + Clone,
    U: ByEid<'a, T> + AssociatedComponentType
{
    fn by_eid(&self) -> &U { &self }
    fn mut_by_eid(&mut self) -> &mut U { self }
    fn component_type() -> ComponentType { U::associated() }
}

#[derive(Debug, PartialEq)]
pub struct ComponentMaps {
    pub coords: HashMap<usize, Coordinates>,
    pub blocking: HashMap<usize, BlockingType>,
    pub decision_timers: HashMap<usize, Timer>,
    pub ais: HashMap<usize, Ai>,
    pub states: HashMap<usize, u32>,
    pub renders: HashMap<usize, Render>,
    pub owner: HashMap<usize, usize>,
    pub alignments: HashMap<usize, AlignmentType>,
    pub healths: HashMap<usize, i32>
}

impl ComponentMaps {
    pub fn new() -> ComponentMaps {
        ComponentMaps {
            coords: HashMap::new(),
            blocking: HashMap::new(),
            decision_timers: HashMap::new(),
            ais: HashMap::new(),
            states: HashMap::new(),
            renders: HashMap::new(),
            owner: HashMap::new(),
            alignments: HashMap::new(),
            healths: HashMap::new()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Components {
    pub coords: CoordinateComponents,
    pub blocking: Blocking,
    pub decision_timers: DecisionTimers,
    pub ais: Ais,
    pub states: States,
    pub renders: Renders,
    pub owner: Owner,
    pub alignments: Alignments,
    pub healths: Healths
}

impl Components {
    pub fn initialize(capacity: usize) -> Components {
        Components {
            coords: CoordinateComponents::initialize(capacity),
            blocking: Blocking::initialize(capacity),
            decision_timers: DecisionTimers::initialize(capacity),
            ais: Ais::initialize(capacity),
            states: States::initialize(capacity),
            renders: Renders::initialize(capacity),
            owner: Owner::initialize(capacity),
            alignments: Alignments::initialize(capacity),
            healths: Healths::initialize(capacity)
        }
    }

    pub fn to_maps(&self) -> ComponentMaps {
        ComponentMaps {
            coords: self.coords.to_map(),
            blocking: self.blocking.to_map(),
            decision_timers: self.decision_timers.to_map(),
            ais: self.ais.to_map(),
            states: self.states.to_map(),
            renders: self.renders.to_map(),
            owner: self.owner.to_map(),
            alignments: self.alignments.to_map(),
            healths: self.healths.to_map()
        }
    } 
}
