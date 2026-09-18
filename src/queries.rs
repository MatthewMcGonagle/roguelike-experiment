use crate::containers::*;
use crate::data::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Queries {
    pub coords_query: CoordinatesQuery,
    pub owns: VecIndexedByEid<Vec<usize>>,
    pub component_types: VecIndexedByEid<Vec<ComponentType>>,
    pub alignments: HashMap<AlignmentType, HashSet<u32>>
}

impl Queries {
    pub fn initialize(capacity: usize, coord_width: usize, coord_height: usize) -> Queries {
        Queries {
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height),
            owns: VecIndexedByEid::initialize(capacity),
            component_types: VecIndexedByEid::initialize(capacity),
            alignments: HashMap::from([
                (AlignmentType::User, HashSet::with_capacity(capacity)),
                (AlignmentType::Neutral, HashSet::with_capacity(capacity)),
                (AlignmentType::HostileToUser, HashSet::with_capacity(capacity))
                ])
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

    pub fn add(&mut self, x: usize, y: usize, space_data: SpaceData) -> Result<(), Errors> {
        let space = self.get_mut(x, y)?;
        match space {
            SpaceData::Empty => {
                *space = space_data;
                Ok(())
            },
            _ => Err(Errors::SpaceAlreadyNonempty) 
        }
    }
}
