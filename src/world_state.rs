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
    let parsed_lines = world_string.split('\n').enumerate().map(|(row_i, line)|
        parse_line(row_i, line)
    );
    let nested = parsed_lines.collect::<Result<Vec<_>, _>>()?;
    Ok(nested.into_iter().flatten().collect())
}
