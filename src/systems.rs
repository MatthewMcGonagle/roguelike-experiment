use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;

pub fn draw_squares(coords: &CoordinateComponents, renders: &Renders, canvas: &mut Canvas<Window>) { 
    let s_width = 100;

    for (e_id, c) in coords.values.values.iter().enumerate() {
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
