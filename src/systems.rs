use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;

use crate::components::*;

pub fn draw_squares(coords: &CoordinateComponents, canvas: &mut Canvas<Window>) { 
    let s_width = 100;
    let s_color = Color::RGB(125, 125, 125);

    canvas.set_draw_color(s_color);
    for c in coords.values.iter() {
        let square = Rect::new(c.x, c.y, s_width, s_width);
        _ = canvas.fill_rect(square);
    }
}
