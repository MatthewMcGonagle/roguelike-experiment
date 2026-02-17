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

    let coord_width: usize = 16;
    let coord_height: usize = 12;
    let coord_scale: u32 = 50;
    let display = Display { width: (coord_width as u32) * coord_scale, height: (coord_height as u32) * coord_scale, coord_scale: coord_scale };
    let window = video_subsystem.window("rust-sdl3 demo", display.width, display.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut components = Components::initialize(display, coord_width, coord_height);
    let mut entities = Entities::initialize();

    entities.add_timed_square_creator(&mut components.e_components, Coordinates { x: 0, y: 0 }, 50); 
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 2, y: 2 }, 10, Ai::ShiftX, Render { color: Color::RGB(0, 0, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 6, y: 4 }, 15, Ai::ShiftY, Render { color: Color::RGB(255, 0, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 8, y: 6 }, 25, Ai::ShiftX, Render { color: Color::RGB(0, 255, 0) });
    entities.add_timed_square(&mut components.e_components, Coordinates { x: 2, y: 8 }, 35, Ai::ShiftY, Render { color: Color::RGB(0, 0, 255) });

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
      
        draw_squares(&components.e_components.coords, components.display.coord_scale, &components.e_components.renders, &mut canvas);
        update_timers(&mut components.e_components.action_timers, &mut components.actions_ready);
        do_actions(&mut components, &mut entities);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
