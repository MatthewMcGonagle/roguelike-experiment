use crate::components::*;

#[derive(Debug, PartialEq)]
pub struct Queries {
    pub coords_query: CoordinatesQuery
}

impl Queries {
    pub fn initialize(coord_width: usize, coord_height: usize) -> Queries {
        Queries {
            coords_query: CoordinatesQuery::initialize(coord_width, coord_height)
        }
    }
}
