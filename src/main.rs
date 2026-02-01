extern crate sdl3;

mod components;
mod systems;

use components::*;
use systems::*;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use std::time::Duration;

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
    let mut components = Components {
        coords: CoordinateComponents {
            values: [Coordinates{x: 0, y: 0}, Coordinates{x: 200, y: 300}, Coordinates{x: 300, y: 500}, Coordinates{x: 400, y: 400}],
        },
        action_timers: ActionTimers { 
            values: [Timer { time: 10, reset: 5 }, Timer { time: 10, reset: 7 }, Timer { time: 13, reset: 13 }, Timer { time: 10, reset: 17}]
        }
    };

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
      
        draw_squares(&components.coords, &mut canvas);
        components.action_timers.update();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
