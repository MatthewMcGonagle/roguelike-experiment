mod containers;
pub mod for_entities;

use crate::data::*;
use for_entities::*;
use std::collections::HashMap;

pub trait Component<'a, T> where T: 'a {
    fn get(&self, e_id: usize) -> Option<&T>;
    fn get_mut(&mut self, e_id: usize) -> Option<&mut T>;
    fn add(&mut self, e_id: usize, value: T) -> ComponentType;
    fn remove(&mut self, e_id: usize);
    fn iter_w_eid(&'a self) -> impl Iterator<Item = (usize, &'a Option<T>)>;
    fn iter_mut_w_eid(&'a mut self) -> impl Iterator<Item = (usize, &'a mut Option<T>)>;
    fn to_map(&self) -> HashMap<usize, T>;
}

#[derive(Debug, PartialEq)]
pub struct ComponentMaps {
    pub component_types: HashMap<usize, Vec<ComponentType>>,
    pub coords: HashMap<usize, Coordinates>,
    pub blocking: HashMap<usize, BlockingType>,
    pub decision_timers: HashMap<usize, Timer>,
    pub ais: HashMap<usize, Ai>,
    pub states: HashMap<usize, u32>,
    pub renders: HashMap<usize, Render>,
    pub owns: HashMap<usize, Vec<usize>>,
    pub owner: HashMap<usize, usize>,
    pub alignments: HashMap<usize, AlignmentType>,
    pub healths: HashMap<usize, i32>
}

impl ComponentMaps {
    pub fn new() -> ComponentMaps {
        ComponentMaps {
            component_types: HashMap::new(),
            coords: HashMap::new(),
            blocking: HashMap::new(),
            decision_timers: HashMap::new(),
            ais: HashMap::new(),
            states: HashMap::new(),
            renders: HashMap::new(),
            owns: HashMap::new(),
            owner: HashMap::new(),
            alignments: HashMap::new(),
            healths: HashMap::new()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Components {
    pub component_types: ComponentTypes,
    pub coords: CoordinateComponents,
    pub coords_query: CoordinatesQuery,
    pub blocking: Blocking,
    pub decision_timers: DecisionTimers,
    pub ais: Ais,
    pub states: States,
    pub renders: Renders,
    pub owns: Owns,
    pub owner: Owner,
    pub alignments: Alignments,
    pub healths: Healths
}

impl Components {
    pub fn initialize(capacity: usize, coord_width: usize, coord_height: usize) -> Components {
        Components {
            component_types: ComponentTypes::initialize(capacity),
            coords: CoordinateComponents::initialize(capacity),
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height),
            blocking: Blocking::initialize(capacity),
            decision_timers: DecisionTimers::initialize(capacity),
            ais: Ais::initialize(capacity),
            states: States::initialize(capacity),
            renders: Renders::initialize(capacity),
            owns: Owns::initialize(capacity),
            owner: Owner::initialize(capacity),
            alignments: Alignments::initialize(capacity),
            healths: Healths::initialize(capacity)
        }
    }

    pub fn to_maps(&self) -> ComponentMaps {
        ComponentMaps {
            component_types: self.component_types.to_map(),
            coords: self.coords.to_map(),
            blocking: self.blocking.to_map(),
            decision_timers: self.decision_timers.to_map(),
            ais: self.ais.to_map(),
            states: self.states.to_map(),
            renders: self.renders.to_map(),
            owns: self.owns.to_map(),
            owner: self.owner.to_map(),
            alignments: self.alignments.to_map(),
            healths: self.healths.to_map()
        }
    } 
}
