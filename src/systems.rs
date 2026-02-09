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

fn update_timer(e_id: usize, timer: &mut Timer) -> Option<usize> {
    match timer.update() {
        TimerResult::Tick => None,
        TimerResult::Reset => Some(e_id)
    }
}

pub fn update_timers(action_timers: &mut ActionTimers, ais: &mut Ais) {
    let ids_of_resets = action_timers.values.iter_mut_w_eid().flat_map(
        |(e_id, maybeTimer)| maybeTimer.as_mut().and_then(
            |timer| update_timer(e_id, timer)
        )
    );
    // let actions = ids_of_resets.collect();
}
