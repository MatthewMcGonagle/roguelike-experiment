use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;
use crate::entities::Entities;

pub fn draw_squares(coords: &CoordinateComponents, coord_scale: usize, renders: &Renders, canvas: &mut Canvas<Window>) { 
    for (e_id, c) in coords.iter_w_eid() {
        match c {
            None => (),
            Some(c) => {
                match renders.get(e_id) {
                    None => (),
                    Some(render) => {
                        let square = Rect::new((c.x * coord_scale) as i32, (c.y * coord_scale) as i32, coord_scale as u32, coord_scale as u32);
                        canvas.set_draw_color(render.color);
                        _ = canvas.fill_rect(square);
                    }
                }

            }

        }
    }
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

fn add_available_square(e_id: usize, e_components: &mut EntityComponents, entities: &mut Entities) {
    let square_ai = match e_components.states.get(e_id).unwrap() {
        0 => Ai::ShiftX,
        _ => Ai::ShiftY
    };
    e_components.states.get_mut(e_id).map(|s| *s = (*s + 1u32) % 2);
    if entities.n_free_ids() >= 2 {
        let maybe_spawned_e_id = entities.add_timed_square(
            e_components,
            e_components.coords.get(e_id).unwrap().clone(),
            10,
            square_ai,
            AlignmentType::User,
            1,
            Render { color: Color::RGB(255, 255, 255) }
        );
        maybe_spawned_e_id.and_then(|s_e_id| entities.add_kill_timer(e_components, 140, s_e_id));
    }
}

fn kill_others_and_self(e_id: usize, e_components: &mut EntityComponents, entities: &mut Entities) {
    let targets: Vec<usize> = e_components.targets.get(e_id).into_iter().flat_map(|ts| ts.clone()).collect();
    for target in targets {
        entities.remove(target, e_components);
    }
    entities.remove(e_id, e_components);
}

fn make_decision(e_id: usize, ai: &Ai) -> Result<Action, Errors> {
    match ai {
        Ai::ShiftX => Ok(Action::Move(e_id, Direction::Right)),
        Ai::ShiftY => Ok(Action::Move(e_id, Direction::Down)),
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

pub fn make_decisions(decisions_ready: &mut DecisionsReady, ais: &Ais, planned_actions: &mut PlannedActions) -> Result<Option<LoopState>, Errors> {
    let mut e_id_needs_user_decision: Option<usize> = None;

    while !decisions_ready.values.is_empty() && e_id_needs_user_decision.is_none() {
        let e_id = decisions_ready.values.pop().unwrap();
        let ai = ais.get(e_id).ok_or(Errors::MissingExpectedEid)?;
        match ai {
            Ai::User => {
                e_id_needs_user_decision = Some(e_id);
            },
            _ => {
                let action = make_decision(e_id, &ai)?;
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

fn decide_user_direction_action(e_id: usize, direction: Direction, e_components: &EntityComponents) -> Result<Action, Errors> {
    let shift = shift_of(&direction);
    let user_coords = e_components.coords.get(e_id).ok_or(Errors::MissingExpectedEid)?;
    let target_coords = target_of_shift(user_coords, e_components.coords_query.coord_width, e_components.coords_query.coord_height, shift);
    match e_components.coords_query.get(target_coords.x, target_coords.y)? {
        SpaceData::Empty => Ok(Action::Move(e_id, direction)),
        SpaceData::HasEid(target_eid) => match e_components.alignments.get(*target_eid) {
            Some(AlignmentType::HostileToUser) => Ok(Action::Attack(e_id, *target_eid)),
            _ => Ok(Action::Wait)
        }
    }
}

pub fn make_user_decision(e_id: usize, key_press: &Keycode, planned_actions: &mut PlannedActions, e_components: &EntityComponents) ->
    Result<Option<LoopState>, Errors> {
    let coords = e_components.coords.get(e_id);
    let loop_state = match key_press {
        Keycode::J => {
            let action = decide_user_direction_action(e_id, Direction::Down, e_components)?;
            planned_actions.values.push(action);
            println!("Pressed J");
            Some(LoopState::MakeDecisions)
        },
        Keycode::K => {
            let action = decide_user_direction_action(e_id, Direction::Up, e_components)?;
            planned_actions.values.push(action);
            println!("Pressed K");
            Some(LoopState::MakeDecisions)
        },
        Keycode::L => {
            let action = decide_user_direction_action(e_id, Direction::Right, e_components)?;
            planned_actions.values.push(Action::Move(e_id, Direction::Right));
            println!("Pressed L");
            Some(LoopState::MakeDecisions)
        },
        Keycode::H => {
            let action = decide_user_direction_action(e_id, Direction::Left, e_components)?;
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

fn do_attack(e_id: usize, target_id: usize, e_components: &mut EntityComponents) -> Option<Reaction> {
    println!("{e_id} attacks {target_id}");
    let maybe_h = e_components.healths.get_mut(target_id);
    match maybe_h {
        Some(h) => {
            *h -= 1;
            println!("    health now {h}");
            if *h <= 0 { Some(Reaction::Kill(e_id)) }
            else { None }
        },
        None => {
            println!("    but has no health");
            None
        }
    }
}

fn do_action(action: Action, e_components: &mut EntityComponents, entities: &mut Entities) -> Option<Reaction> {
    match action {
        Action::Move(e_id, direction) => {
            let w = e_components.coords_query.coord_width.clone();
            let h = e_components.coords_query.coord_height.clone();
            shift(e_id, &mut e_components.blocking, &mut e_components.coords, &mut e_components.coords_query, w, h, shift_of(&direction));
            None
        },
        Action::Spawn(e_id) => { add_available_square(e_id, e_components, entities); None },
        Action::Kill(e_id) => { kill_others_and_self(e_id, e_components, entities); None },
        Action::Attack(e_id, target_id) => do_attack(e_id, target_id, e_components),
        _ => None
    }
}

pub fn do_actions(components: &mut Components, entities: &mut Entities) {
    while !components.planned_actions.values.is_empty() {
        let action = components.planned_actions.values.pop().unwrap();
        match do_action(action, &mut components.e_components, entities) {
            Some(reaction) => components.reactions_ready.values.push(reaction),
            None => ()
        }
    }
}

fn react_to_no_health(e_id: usize, health: i32, e_components: &mut EntityComponents) {
}

pub fn do_reactions(reactions_ready: &mut ReactionsReady, e_components: &mut EntityComponents, entities: &mut Entities) {
}
