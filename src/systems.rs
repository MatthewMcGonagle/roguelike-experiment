use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;
use crate::entities::Entities;

pub fn draw_squares(coords: &CoordinateComponents, coord_scale: u32, renders: &Renders, canvas: &mut Canvas<Window>) { 
    for (e_id, c) in coords.values.iter_w_eid() {
        match c {
            None => (),
            Some(c) => {
                match renders.get(e_id) {
                    None => (),
                    Some(render) => {
                        let square = Rect::new(c.x * (coord_scale as i32), c.y * (coord_scale as i32), coord_scale, coord_scale);
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
    for (e_id, maybe_timer) in action_timers.values.iter_mut_w_eid() {
        if let Some(t) = maybe_timer.as_mut() { 
            if update_timer(t) {
                actions_ready.add(e_id);
            }
        }
    }
}

fn shift_x(coords: Option<&mut Coordinates>, coord_width: u32) {
    coords.map(|c| c.x = (c.x + 1) % (coord_width as i32));
}

fn shift_y(coords: Option<&mut Coordinates>, coord_height: u32) {
    coords.map(|c| c.y = (c.y + 1) % (coord_height as i32));
}

fn add_available_square(e_id: usize, e_components: &mut EntityComponents, entities: &mut Entities) {
    let square_ai = match e_components.states.values.get(e_id).unwrap() {
        0 => Ai::ShiftX,
        _ => Ai::ShiftY
    };
    e_components.states.values.get_mut(e_id).map(|s| *s = (*s + 1u32) % 2);
    entities.add_timed_square(
        e_components,
        e_components.coords.values.get(e_id).unwrap().clone(),
        10,
        square_ai,
        Render { color: Color::RGB(255, 255, 255) }
    ).unwrap_or(())
}

fn do_action(e_id: usize, display: &Display, ai: Ai, e_components: &mut EntityComponents, entities: &mut Entities) {
    match ai {
        Ai::ShiftX => shift_x(e_components.coords.values.get_mut(e_id), display.coord_width()),
        Ai::ShiftY => shift_y(e_components.coords.values.get_mut(e_id), display.coord_height()),
        Ai::AddAvailableSquare => add_available_square(e_id, e_components, entities),
        Ai::Kill => () 
    }
}

pub fn do_actions(components: &mut Components, entities: &mut Entities) {
    for e_id in components.actions_ready.values.iter() {
        let maybe_ai: Option<Ai> = components.e_components.ais.values.get(*e_id).cloned();
        maybe_ai.map(|ai| do_action(*e_id, &components.display, ai, &mut components.e_components, entities));
    }
    components.actions_ready.values.clear();
}
