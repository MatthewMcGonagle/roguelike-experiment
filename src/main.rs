extern crate sdl3;

mod entities;
mod components;
mod systems;

use entities::*;
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
    let mut components = Components::initialize();
    let mut entities = Entities::initialize();

    entities.add_timed_square_creator(&mut components.e_components, Coordinates { x: 0, y: 0 }, 100); 
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 100, y: 200 }, 20, Ai::ShiftX, Render { color: Color::RGB(0, 0, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 300, y: 200 }, 30, Ai::ShiftX, Render { color: Color::RGB(255, 0, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 400, y: 400 }, 50, Ai::ShiftX, Render { color: Color::RGB(0, 255, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 100, y: 400 }, 70, Ai::ShiftX, Render { color: Color::RGB(0, 0, 255) });

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
      
        draw_squares(&components.e_components.coords, &components.e_components.renders, &mut canvas);
        update_timers(&mut components.e_components.action_timers, &mut components.actions_ready);
        do_actions(&mut components, &mut entities);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
