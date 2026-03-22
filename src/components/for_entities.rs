use super::*;

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
    fn component_type() -> ComponentType { ComponentType::ComponentTypeList }
}

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
    fn component_type() -> ComponentType { ComponentType::Coordinates}
}

pub struct CoordinatesQuery {
    pub coord_width: usize,
    pub coord_height: usize,
    values: Vec<SpaceData>
}

impl CoordinatesQuery {
    pub fn initialize(coord_width: usize, coord_height: usize) -> CoordinatesQuery {
        let mut the_values: Vec<SpaceData> = Vec::with_capacity(coord_width * coord_height);
        the_values.resize(coord_width * coord_height, SpaceData::Empty);

        CoordinatesQuery {
            coord_width: coord_width,
            coord_height: coord_height,
            values: the_values 
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&SpaceData, Errors> {
        self.values.get(y * self.coord_width + x).ok_or(Errors::CoordinateMissing)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut SpaceData, Errors> {
        self.values.get_mut(y * self.coord_width + x).ok_or(Errors::CoordinateMissing)
    }

    pub fn add(&mut self, x: usize, y: usize, space_data: SpaceData) -> Result<ComponentType, Errors> {
        let space = self.get_mut(x, y)?;
        match space {
            SpaceData::Empty => {
                *space = space_data;
                Ok(ComponentType::CoordinatesQuery)
            },
            _ => Err(Errors::SpaceAlreadyNonempty) 
        }
    }
}

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
    fn component_type() -> ComponentType { ComponentType::Blocking }
}

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
    fn component_type() -> ComponentType { ComponentType::DecisionTimer }
}

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
    fn component_type() -> ComponentType { ComponentType::Ai }
}

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
    fn component_type() -> ComponentType { ComponentType::State }
}

pub struct DecisionsReady {
    pub values: Vec<usize>
}

impl DecisionsReady {
    pub fn initialize(capacity: usize) -> DecisionsReady {
        DecisionsReady { values: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, e_id: usize) { self.values.push(e_id) }
}

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
    fn component_type() -> ComponentType { ComponentType::Render }
}

pub struct Targets {
    values: VecIndexedByEid<Vec<usize>>
}

impl Targets {
    pub fn initialize(capacity: usize) -> Targets {
        Targets { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<Vec<usize>> for Targets {
    fn the_values(&self) -> &VecIndexedByEid<Vec<usize>> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Vec<usize>> { &mut self.values }
    fn component_type() -> ComponentType { ComponentType::Target }
}

// If we kill this e_id then we need to appropriately updates other entities that target this one.
pub struct TargetedBy {
    values: VecIndexedByEid<Vec<usize>>
}

impl TargetedBy {
    pub fn initialize(capacity: usize) -> TargetedBy {
        TargetedBy { values: VecIndexedByEid::initialize(capacity) } 
    }
}

impl UsesVecIndexedByEid<Vec<usize>> for TargetedBy {
    fn the_values(&self) -> &VecIndexedByEid<Vec<usize>> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Vec<usize>> { &mut self.values }
    fn component_type() -> ComponentType { ComponentType::TargetedBy }
}

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
    fn component_type() -> ComponentType { ComponentType::Alignment }
}

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
    fn component_type() -> ComponentType { ComponentType::Health }
}
