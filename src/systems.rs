use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;
use crate::entities::Entities;

pub fn draw_squares(coords: &CoordinateComponents, renders: &Renders, canvas: &mut Canvas<Window>) { 
    let s_width = 100;

    for (e_id, c) in coords.values.iter_w_eid() {
        match c {
            None => (),
            Some(c) => {
                match renders.get(e_id) {
                    None => (),
                    Some(render) => {
                        let square = Rect::new(c.x, c.y, s_width, s_width);
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

fn do_action(e_id: usize, ai: Ai, others: &mut OtherComponents, entities: &mut Entities) {
    match ai {
        Ai::ShiftX => (),
        Ai::AddAvailableSquare => entities.add_timed_square(
            others,
            others.coords.values.get(e_id).unwrap().clone(),
            100,
            Ai::ShiftX,
            Render { color: Color::RGB(0, 0, 0) }
        ).unwrap_or(())
    }
}

pub fn do_actions(components: &mut Components, entities: &mut Entities) {
    for e_id in components.actions_ready.values.iter() {
        let maybe_ai: Option<Ai> = components.others.ais.values.get(*e_id).cloned();
        maybe_ai.map(|ai| do_action(*e_id, ai, &mut components.others, entities));
    }
    components.actions_ready.values.clear();
}
