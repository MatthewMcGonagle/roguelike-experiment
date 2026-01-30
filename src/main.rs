extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use std::time::Duration;
use sdl3::video::Window;

struct Coordinates {
    x: i32,
    y: i32,
}

struct CoordinateComponents {
    values: [Coordinates; 4],
}

struct Timer { time: u32, reset: u32 }

impl Timer {
    fn update(&mut self) {
        self.time = self.time - 1;
        if self.time <= 0 {
            self.time = self.reset;
        }
    }
}

struct ActionTimers {
    values: [Timer; 4]
}

impl ActionTimers {
    fn update(&mut self) {
        for timer in self.values.iter_mut() {
            timer.update();
        }
    }
}

struct Components {
    coords: CoordinateComponents,
    action_timers: ActionTimers
}

fn draw_squares(coords: &CoordinateComponents, canvas: &mut Canvas<Window>) { 
    let s_width = 100;
    let s_color = Color::RGB(125, 125, 125);

    canvas.set_draw_color(s_color);
    for c in coords.values.iter() {
        let square = Rect::new(c.x, c.y, s_width, s_width);
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
