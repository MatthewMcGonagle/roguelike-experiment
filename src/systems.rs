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

pub fn update_timers(action_timers: &mut ActionTimers, actions_ready: &mut ActionsReady) {
    for (e_id, maybe_timer) in action_timers.iter_mut_w_eid() {
        if let Some(t) = maybe_timer.as_mut() { 
            if update_timer(t) {
                actions_ready.add(e_id);
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

fn shift_x(e_id: usize, blocking: &mut Blocking, e_coords: &mut CoordinateComponents, c_query: &mut CoordinatesQuery, coord_width: usize) -> Option<()> {
    let coords = e_coords.get(e_id)?;
    let target_coords = Coordinates { x: (coords.x + 1) % coord_width, y: coords.y }; 
    move_coords(e_id, blocking, e_coords, c_query, target_coords)
}

fn shift_y(e_id: usize, blocking: &mut Blocking, e_coords: &mut CoordinateComponents, c_query: &mut CoordinatesQuery, coord_height: usize) -> Option<()> {
    let coords = e_coords.get(e_id)?;
    let target_coords = Coordinates { x: coords.x, y: (coords.y + 1) % coord_height }; 
    move_coords(e_id, blocking, e_coords, c_query, target_coords)
}

fn add_available_square(e_id: usize, e_components: &mut EntityComponents, entities: &mut Entities) -> Option<()> {
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
            Render { color: Color::RGB(255, 255, 255) }
        );
        maybe_spawned_e_id.and_then(|s_e_id| entities.add_kill_timer(e_components, 140, s_e_id));
    }
    Some(())
}

fn kill_others_and_self(e_id: usize, e_components: &mut EntityComponents, entities: &mut Entities) -> Option<()> {
    let targets: Vec<usize> = e_components.targets.values.get(e_id).into_iter().flat_map(|ts| ts.clone()).collect();
    for target in targets {
        entities.remove(target, e_components);
    }
    entities.remove(e_id, e_components);
    Some(())
}

fn do_action(e_id: usize, ai: Ai, e_components: &mut EntityComponents, entities: &mut Entities) -> Option<()> {
    match ai {
        Ai::ShiftX => {
            let w = e_components.coords_query.coord_width.clone();
            shift_x(e_id, &mut e_components.blocking, &mut e_components.coords, &mut e_components.coords_query, w)
        },
        Ai::ShiftY => {
            let h = e_components.coords_query.coord_height.clone();
            shift_y(e_id, &mut e_components.blocking, &mut e_components.coords, &mut e_components.coords_query, h)
        },
        Ai::AddAvailableSquare => add_available_square(e_id, e_components, entities),
        Ai::Kill => kill_others_and_self(e_id, e_components, entities) 
    }
}

pub fn do_actions(components: &mut Components, entities: &mut Entities) {
    for e_id in components.actions_ready.values.iter() {
        let maybe_ai: Option<Ai> = components.e_components.ais.get(*e_id).cloned();
        maybe_ai.map(|ai| do_action(*e_id, ai, &mut components.e_components, entities));
    }
    components.actions_ready.values.clear();
}
