use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;

pub fn draw_squares(coords: &CoordinateComponents, renders: &Renders, canvas: &mut Canvas<Window>) { 
    let s_width = 100;
    let s_color = Color::RGB(125, 125, 125);

    canvas.set_draw_color(s_color);
    for c in coords.values.iter() {
        match c.e_id {
            None => (),
            Some(e_id) => {
                let render = renders.get(e_id);
                let square = Rect::new(c.value.x, c.value.y, s_width, s_width);
                canvas.set_draw_color(render.color);
                _ = canvas.fill_rect(square);
            }

        }
    }
}
