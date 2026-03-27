use sdl3::Error;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;
use crate::components::for_entities::*;
use crate::data::*;
use crate::game_state::*;
use crate::entities::Entities;

fn draw_square(coords: &Coordinates, coord_scale: usize, render: &Render, canvas: &mut Canvas<Window>) -> Result<(), Errors> {
    let square = Rect::new((coords.x * coord_scale) as i32, (coords.y * coord_scale) as i32, coord_scale as u32, coord_scale as u32);
    canvas.set_draw_color(render.color);
    canvas.fill_rect(square).map_err(|e| Errors::SDL3Error(e))
}

pub fn draw_squares(coords: &CoordinateComponents, coord_scale: usize, renders: &Renders, canvas: &mut Canvas<Window>) -> Result<(), Errors> { 
    for (e_id, c) in coords.iter_w_eid() {
        match c {
            None => (),
            Some(c) => {
                match renders.get(e_id) {
                    None => (),
                    Some(render) => draw_square(c, coord_scale, render, canvas)?
                }
            }
        }
    }
    Ok(())
}

fn update_timer(timer: &mut Timer) -> bool {
    match timer.update() {
        TimerResult::Tick => false,
        TimerResult::Reset => true
    }
}

pub fn update_timers(decision_timers: &mut DecisionTimers, decisions_ready: &mut DecisionsReady) {
    for (e_id, maybe_timer) in decision_timers.iter_mut_w_eid() {
        if let Some(t) = maybe_timer.as_mut() { 
            if update_timer(t) {
                decisions_ready.add(e_id);
            }
        }
    }
}

fn move_coords_for_blocking(e_id: usize, e_coords: &mut CoordinateComponents, c_query: &mut CoordinatesQuery, target_coords: Coordinates) -> Option<()> {
    let target_space = c_query.get_mut(target_coords.x, target_coords.y).ok()?;
    match target_space {
        SpaceData::Empty => {
            let e_coords = e_coords.get_mut(e_id)?;
            *target_space = SpaceData::HasEid(e_id);
            let origin = c_query.get_mut(e_coords.x, e_coords.y).ok()?;
            *e_coords = target_coords;
            *origin = SpaceData::Empty;
            Some(())
        }
        SpaceData::HasEid(other_e_id) => { println!("{e_id} BLOCKED by {other_e_id}"); None }
    }
}

fn move_coords(
    e_id: usize, blocking: &mut Blocking, e_coords: &mut CoordinateComponents, c_query: &mut CoordinatesQuery, target_coords: Coordinates
    ) -> Option<()> {
    if blocking.get(e_id).map(|b| b.clone() == BlockingType::Movement).unwrap_or(false) {
        move_coords_for_blocking(e_id, e_coords, c_query, target_coords)
    } else {
        let coords = e_coords.get_mut(e_id)?;
        *coords = target_coords;
        Some(())
    }
}

fn target_of_shift(coords: &Coordinates, coord_width: usize, coord_height: usize, shift: (i32, i32)) -> Coordinates {
    let (shift_x, shift_y) = shift;
    let target_x_no_mod: i32 = (coords.x as i32) + shift_x;
    let target_y_no_mod: i32 = (coords.y as i32) + shift_y;
    let target_x = if target_x_no_mod < 0 { target_x_no_mod + (coord_width as i32) } else { target_x_no_mod };
    let target_y = if target_y_no_mod < 0 { target_y_no_mod + (coord_height as i32) } else { target_y_no_mod };
    Coordinates { x: (target_x as usize) % coord_width, y: (target_y as usize) % coord_height }
}

fn shift(
    e_id: usize, blocking: &mut Blocking, e_coords: &mut CoordinateComponents, c_query: &mut CoordinatesQuery, coord_width: usize, coord_height: usize,
    shift: (i32, i32)
    ) -> Option<()> {
    let coords = e_coords.get(e_id)?;
    let target_coords = target_of_shift(coords, coord_width, coord_height, shift);
    move_coords(e_id, blocking, e_coords, c_query, target_coords)
}

fn add_available_square(e_id: usize, components: &mut Components, entities: &mut Entities) {
    let square_ai = match components.states.get(e_id).unwrap() {
        0 => Ai::ShiftX,
        _ => Ai::ShiftY
    };
    components.states.get_mut(e_id).map(|s| *s = (*s + 1u32) % 2);
    if entities.n_free_ids() >= 2 {
        let maybe_spawned_e_id = entities.add_timed_square(
            components,
            components.coords.get(e_id).unwrap().clone(),
            10,
            square_ai,
            AlignmentType::User,
            1,
            Render { color: Color::RGB(255, 255, 255) }
        );
        maybe_spawned_e_id.and_then(|s_e_id| entities.add_kill_timer(components, 140, s_e_id));
    }
}

fn kill_others_and_self(e_id: usize, components: &mut Components, entities: &mut Entities) {
    let targets: Vec<usize> = components.targets.get(e_id).into_iter().flat_map(|ts| ts.clone()).collect();
    for target in targets {
        entities.remove(target, components);
    }
    entities.remove(e_id, components);
}

fn decide_move_or_attack(e_id: usize, direction: Direction, components: &Components) -> Result<Action, Errors> {
    let (shift_x, shift_y) = shift_of(&direction);
    let coords = components.coords.get(e_id).ok_or(Errors::MissingExpectedEid)?;
    let coord_width = components.coords_query.coord_width;
    let coord_height = components.coords_query.coord_height;
    let target_coords = target_of_shift(coords, coord_width, coord_height, (shift_x, shift_y));
    let space = components.coords_query.get(target_coords.x, target_coords.y)?;
    let action = match space {
        SpaceData::Empty => Action::Move(e_id, direction),
        SpaceData::HasEid(target_id) => {
            match (components.alignments.get(e_id), components.alignments.get(*target_id)) {
                (Some(AlignmentType::HostileToUser), Some(AlignmentType::User)) => Action::Attack(e_id, *target_id),
                (Some(AlignmentType::User), Some(AlignmentType::HostileToUser)) => Action::Attack(e_id, *target_id),
                _ => Action::Wait
            }
        }
    };
    Ok(action)
}

fn make_decision(e_id: usize, ai: &Ai, components: &Components) -> Result<Action, Errors> {
    match ai {
        Ai::ShiftX => decide_move_or_attack(e_id, Direction::Right, components),
        Ai::ShiftY => decide_move_or_attack(e_id, Direction::Down, components),
        Ai::AddAvailableSquare => Ok(Action::Spawn(e_id)),
        Ai::Kill => Ok(Action::Kill(e_id)),
        Ai::User => Err(Errors::NotExpectingAiForUser)
    }
}

fn shift_of(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Up => (0, -1)
    }
}

pub fn make_decisions( 
    decisions_ready: &mut DecisionsReady, components: &Components, planned_actions: &mut PlannedActions
    ) -> Result<Option<LoopState>, Errors> {
    let mut e_id_needs_user_decision: Option<usize> = None;

    while !decisions_ready.values.is_empty() && e_id_needs_user_decision.is_none() {
        let e_id = decisions_ready.values.pop().unwrap();
        let ai = components.ais.get(e_id).ok_or(Errors::MissingExpectedEid)?;
        match ai {
            Ai::User => {
                e_id_needs_user_decision = Some(e_id);
            },
            _ => {
                let action = make_decision(e_id, &ai, components)?;
                planned_actions.values.push(action);
            }
        }
    }

    if e_id_needs_user_decision.is_some() {
        Ok(Some(LoopState::User(e_id_needs_user_decision.unwrap())))
    } else {
        Ok(None)
    }
}

fn decide_user_direction_action(e_id: usize, direction: Direction, components: &Components) -> Result<Action, Errors> {
    let shift = shift_of(&direction);
    let user_coords = components.coords.get(e_id).ok_or(Errors::MissingExpectedEid)?;
    let target_coords = target_of_shift(user_coords, components.coords_query.coord_width, components.coords_query.coord_height, shift);
    match components.coords_query.get(target_coords.x, target_coords.y)? {
        SpaceData::Empty => Ok(Action::Move(e_id, direction)),
        SpaceData::HasEid(target_eid) => match components.alignments.get(*target_eid) {
            Some(AlignmentType::HostileToUser) => Ok(Action::Attack(e_id, *target_eid)),
            _ => Ok(Action::Wait)
        }
    }
}

pub fn make_user_decision(e_id: usize, key_press: &Keycode, planned_actions: &mut PlannedActions, components: &Components) ->
    Result<Option<LoopState>, Errors> {
    let coords = components.coords.get(e_id);
    let loop_state = match key_press {
        Keycode::J => {
            let action = decide_user_direction_action(e_id, Direction::Down, components)?;
            planned_actions.values.push(action);
            println!("Pressed J");
            Some(LoopState::MakeDecisions)
        },
        Keycode::K => {
            let action = decide_user_direction_action(e_id, Direction::Up, components)?;
            planned_actions.values.push(action);
            println!("Pressed K");
            Some(LoopState::MakeDecisions)
        },
        Keycode::L => {
            let action = decide_user_direction_action(e_id, Direction::Right, components)?;
            planned_actions.values.push(action);
            println!("Pressed L");
            Some(LoopState::MakeDecisions)
        },
        Keycode::H => {
            let action = decide_user_direction_action(e_id, Direction::Left, components)?;
            planned_actions.values.push(action);
            println!("Pressed H");
            Some(LoopState::MakeDecisions)
        },
        Keycode::Period => {
            println!("Wait");
            Some(LoopState::MakeDecisions)
        },
        _ => None
    };
    Ok(loop_state)
}

fn do_attack(e_id: usize, target_id: usize, components: &mut Components) -> Option<Reaction> {
    println!("{e_id} attacks {target_id}");
    let maybe_h = components.healths.get_mut(target_id);
    match maybe_h {
        Some(h) => {
            *h -= 1;
            println!("    health now {h}");
            if *h <= 0 { Some(Reaction::Kill(target_id)) }
            else { None }
        },
        None => {
            println!("    but has no health");
            None
        }
    }
}

fn do_action(action: Action, to_kill: &mut ToKill, components: &mut Components, entities: &mut Entities) -> Option<Reaction> {
    match action {
        Action::Move(e_id, direction) => {
            let w = components.coords_query.coord_width.clone();
            let h = components.coords_query.coord_height.clone();
            shift(e_id, &mut components.blocking, &mut components.coords, &mut components.coords_query, w, h, shift_of(&direction));
            None
        },
        Action::Spawn(e_id) => { add_available_square(e_id, components, entities); None },
        Action::Kill(e_id) => { to_kill.values.push(e_id); None },
        Action::Attack(e_id, target_id) => do_attack(e_id, target_id, components),
        _ => None
    }
}

pub fn do_actions(game_state: &mut GameState, entities: &mut Entities) {
    while !game_state.planned_actions.values.is_empty() {
        let action = game_state.planned_actions.values.pop().unwrap();
        match do_action(action, &mut game_state.to_kill, &mut game_state.components, entities) {
            Some(reaction) => game_state.reactions_ready.values.push(reaction),
            None => ()
        }
    }
}

fn do_reaction(reaction: Reaction, to_kill: &mut ToKill, components: &mut Components, entities: &mut Entities) {
    match reaction {
        Reaction::Kill(e_id) => to_kill.values.push(e_id)
    }
}

pub fn do_reactions(reactions_ready: &mut ReactionsReady, to_kill: &mut ToKill, components: &mut Components, entities: &mut Entities) {
    while !reactions_ready.values.is_empty() {
        let reaction = reactions_ready.values.pop().unwrap();
        do_reaction(reaction, to_kill, components, entities);
    }
}

pub fn do_killings(to_kill: &mut ToKill, components: &mut Components, entities: &mut Entities) {
    while !to_kill.values.is_empty() {
        let e_id = to_kill.values.pop().unwrap();
        kill_others_and_self(e_id, components, entities);
    }
}

