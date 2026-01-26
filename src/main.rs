extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use std::time::Duration;
use sdl3::video::Window;

fn draw_squares(coords: [(i32, i32); 4], canvas: &mut Canvas<Window>) { 
    let s_width = 100;
    // let square = Rect::new(300, 200, s_width, s_width);
    let s_color = Color::RGB(125, 125, 125);

    canvas.set_draw_color(s_color);
    for &(x, y) in coords.iter() {
        let square = Rect::new(x, y, s_width, s_width);
        _ = canvas.fill_rect(square);
    }
}

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let coords = [(0, 0), (200, 300), (300, 500), (400, 400)];

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
      
        draw_squares(coords, &mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
