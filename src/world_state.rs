use crate::data::*;

pub fn parse_world_state<'a>(world_string: &'a str) -> Result<Vec<WorldState>, Errors> {
    let parse_state = |row_i, col_i, w| {
        match w {
            "s" => Ok(Some(WorldState::Spawner(col_i, row_i, 50))),
            "#" => Ok(Some(WorldState::Wall(col_i, row_i))),
            "." => Ok(None),
            _ => Err(Errors::UnknownWorldState(w.to_string())) 
        }
    };

    let parse_row = |(row_i, l): (_, &'a str)| {
        l.split(' ')
            .filter(|s| *s != "")
            .enumerate()
            .map(move |(col_i, w)| (row_i, col_i, w))
    };

    let split_rows = |ls: &'a str| ls.split('\n').enumerate();

    let world_states = split_rows(world_string).map(parse_row).flatten();

    world_states.map(|(row_i, col_i, w)| parse_state(row_i, col_i, w))
        .collect::<Result<Vec<_>, _>>()
        .map(|maybes|
            maybes.into_iter()
                .flatten()
                .collect::<Vec<WorldState>>())
}
