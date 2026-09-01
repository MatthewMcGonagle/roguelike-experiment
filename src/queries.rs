use crate::components::*;
use crate::containers::*;
use crate::data::*;

#[derive(Debug, PartialEq)]
pub struct Queries {
    pub coords_query: CoordinatesQuery,
    pub owns: Owns,
    pub component_types: ComponentTypes
}

impl Queries {
    pub fn initialize(capacity: usize, coord_width: usize, coord_height: usize) -> Queries {
        Queries {
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height),
            owns: Owns::initialize(capacity),
            component_types: ComponentTypes::initialize(capacity)
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Owns {
    values: VecIndexedByEid<Vec<usize>>
}

impl Owns {
    pub fn initialize(capacity: usize) -> Owns {
        Owns { values: VecIndexedByEid::initialize(capacity) }
    }
}

impl UsesVecIndexedByEid<Vec<usize>> for Owns {
    fn the_values(&self) -> &VecIndexedByEid<Vec<usize>> { & self.values }
    fn mut_values(&mut self) -> &mut VecIndexedByEid<Vec<usize>> { &mut self.values }
}

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
