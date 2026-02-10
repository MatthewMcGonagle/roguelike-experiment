use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;

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
    action_timers.values.iter_mut_w_eid().map(
        |(e_id, maybeTimer)| maybeTimer.as_mut().map(
            |timer| if update_timer(timer) { actions_ready.values.get_mut(e_id).map(|x| *x = true);} 
        )
    );
}

pub fn do_actions(actions_ready: &mut ActionsReady, ais: &Ais) {
    actions_ready.values.iter_mut_w_eid().map(
        |(e_id, maybe_ready)| {
            if maybe_ready.unwrap_or(false) {
                *maybe_ready = Some(false);
            }
        }
    );
}
