use crate::data::*;

fn parse_line(row_i: usize, l: &str) -> Result<Vec<WorldState>, Errors> {
    let states = l.split(' ').filter(|s| *s != "").enumerate().map(|(col_i, w)|
        match w {
            "#" => Ok(WorldState::Wall(col_i, row_i)),
            _ => Err(Errors::UnknownWorldState(w.to_string())) 
        }
    );
    states.collect()
}

pub fn parse_world_state(world_string: &str) -> Result<Vec<WorldState>, Errors> {
    let parse_state = |row_i, col_i, w| {
        match w {
            "#" => Ok(WorldState::Wall(col_i, row_i)),
            _ => Err(Errors::UnknownWorldState(w.to_string())) 
        }
    };

    let world_states = world_string.split('\n').enumerate().map(
        |(row_i, l)| l.split(' ').filter(|s| *s != "").enumerate().map(
            move |(col_i, w)| (row_i, col_i, w))
    ).flatten();
    world_states.map(|(row_i, col_i, w)| parse_state(row_i, col_i, w)).collect() 
}
