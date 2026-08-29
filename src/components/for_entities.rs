use super::*;
use crate::containers::*;

#[derive(Debug, PartialEq)]
pub struct ComponentTypes {
    values: VecIndexedByEid<Vec<ComponentType>>
}

impl ComponentTypes {
    pub fn initialize(e_id_capacity: usize) -> ComponentTypes {
        ComponentTypes { values: VecIndexedByEid::initialize(e_id_capacity) }
    }

    pub fn push(&mut self, e_id: usize, c_type: ComponentType) -> Result<(), Errors> {
        let current = self.values.get_mut(e_id).ok_or(Errors::MissingExpectedEid)?;
        current.push(c_type);
        Ok(())
    }
}

impl UsesVecIndexedByEid<Vec<ComponentType>> for ComponentTypes {
    fn the_values(&self) -> &VecIndexedByEid<Vec<ComponentType>> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Vec<ComponentType>> { &mut self.values }
}

impl AssociatedComponentType for ComponentTypes {
    fn associated() -> ComponentType { ComponentType::ComponentTypeList }
}

#[derive(Debug, PartialEq)]
pub struct CoordinateComponents {
    values: VecIndexedByEid<Coordinates>,
}

impl CoordinateComponents {
    pub fn initialize(capacity: usize) -> CoordinateComponents {
        CoordinateComponents {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<Coordinates> for CoordinateComponents {
    fn the_values(&self) -> &VecIndexedByEid<Coordinates> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Coordinates> { &mut self.values }
}

impl AssociatedComponentType for CoordinateComponents {
    fn associated() -> ComponentType { ComponentType::Coordinates}
}

#[derive(Debug, PartialEq)]
pub struct Blocking {
    values: VecIndexedByEid<BlockingType>
}

impl Blocking {
    pub fn initialize(capacity: usize) -> Blocking {
        Blocking {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<BlockingType> for Blocking {
    fn the_values(&self) -> &VecIndexedByEid<BlockingType> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<BlockingType> { &mut self.values }
}

impl AssociatedComponentType for Blocking {
    fn associated() -> ComponentType { ComponentType::Blocking }
}

#[derive(Debug, PartialEq)]
pub struct DecisionTimers {
    values: VecIndexedByEid<Timer>
}

impl DecisionTimers {
    pub fn initialize(capacity: usize) -> DecisionTimers {
        DecisionTimers {
            values: VecIndexedByEid::initialize(capacity)
        }
    }
}

impl UsesVecIndexedByEid<Timer> for DecisionTimers {
    fn the_values(&self) -> &VecIndexedByEid<Timer> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Timer> { &mut self.values }
}

impl AssociatedComponentType for DecisionTimers {
    fn associated() -> ComponentType { ComponentType::DecisionTimer }
}

#[derive(Debug, PartialEq)]
pub struct Ais {
    values: VecIndexedByEid<Ai>
}

impl Ais {
    pub fn initialize(capacity: usize) -> Ais {
        Ais { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<Ai> for Ais {
    fn the_values(&self) -> &VecIndexedByEid<Ai> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Ai> { &mut self.values }
}

impl AssociatedComponentType for Ais {
    fn associated() -> ComponentType { ComponentType::Ai }
}

#[derive(Debug, PartialEq)]
pub struct States {
    values: VecIndexedByEid<u32>
}

impl States {
    pub fn initialize(capacity: usize) -> States {
        States { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<u32> for States {
    fn the_values(&self) -> &VecIndexedByEid<u32> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<u32> { &mut self.values }
}

impl AssociatedComponentType for States {
    fn associated() -> ComponentType { ComponentType::State }
}

#[derive(Debug, PartialEq)]
pub struct DecisionsReady {
    pub values: Vec<usize>
}

impl DecisionsReady {
    pub fn initialize(capacity: usize) -> DecisionsReady {
        DecisionsReady { values: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, e_id: usize) { self.values.push(e_id) }
}

#[derive(Debug, PartialEq)]
pub struct Renders {
    values: VecIndexedByEid<Render>
}

impl Renders {
    pub fn initialize(capacity: usize) -> Renders {
        Renders { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<Render> for Renders {
    fn the_values(&self) -> &VecIndexedByEid<Render> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Render> { &mut self.values }
}

impl AssociatedComponentType for Renders {
    fn associated() -> ComponentType { ComponentType::Render }
}

#[derive(Debug, PartialEq)]
pub struct Owner {
    values: VecIndexedByEid<usize>
}

impl Owner {
    pub fn initialize(capacity: usize) -> Owner {
        Owner { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<usize> for Owner {
    fn the_values(&self) -> &VecIndexedByEid<usize> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<usize> { &mut self.values }
}

impl AssociatedComponentType for Owner {
    fn associated() -> ComponentType { ComponentType::Owner }
}

#[derive(Debug, PartialEq)]
pub struct Alignments {
    values: VecIndexedByEid<AlignmentType>
}

impl Alignments {
    pub fn initialize(capacity: usize) -> Alignments {
        Alignments { values: VecIndexedByEid::initialize(capacity) } 
    }
}

impl UsesVecIndexedByEid<AlignmentType> for Alignments {
    fn the_values(&self) -> &VecIndexedByEid<AlignmentType> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<AlignmentType> { &mut self.values }
}

impl AssociatedComponentType for Alignments {
    fn associated() -> ComponentType { ComponentType::Alignment }
}

#[derive(Debug, PartialEq)]
pub struct Healths {
    values: VecIndexedByEid<i32>
}

impl Healths {
    pub fn initialize(capacity: usize) -> Healths {
        Healths { values: VecIndexedByEid::initialize(capacity) } 
    }
}

impl UsesVecIndexedByEid<i32> for Healths {
    fn the_values(&self) -> &VecIndexedByEid<i32> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<i32> { &mut self.values }
}

impl AssociatedComponentType for Healths {
    fn associated() -> ComponentType { ComponentType::Health }
}
