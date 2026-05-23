extern crate sdl3;

mod components;
mod data;
mod entities;
mod game_state;
mod state_storage;
mod world_state;
mod systems;

use data::*;
use game_state::*;
use state_storage::*;
use systems::*;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use std::fs;
use std::time::Duration;

pub fn main() {
    match safe_main() {
        Ok(()) => (),
        Err(e) => println!("{:?}", e)
    };
}

pub fn safe_main() -> Result<(), Errors> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let state_store_string = fs::read_to_string("resources/state_storage.toml")
        .expect("State storage file not found.");
    let state_store: state_storage::StateStorage = toml::from_str(&state_store_string).expect("Can't parse toml string.");

    let world_states = world_state::parse_world_state(&state_store.map)?;

    let free_ids_allocation_size = 20;
    let coord_width: usize = 16;
    let coord_height: usize = 12;
    let coord_scale: usize = 50;
    let display = Display { width: (coord_width * coord_scale) as u32, height: (coord_height * coord_scale) as u32, coord_scale: coord_scale };
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
    let mut game_state = GameState::initialize(free_ids_allocation_size, LoopState::RunTimers, display, coord_width, coord_height);
    let mut key_press: Option<Keycode> = None;

    for e_store in state_store.entities {
        game_state.entities.add_entity_storage(&mut game_state.components, e_store)?
    }

    add_world_states(&mut game_state.entities, &mut game_state.components, world_states)?;
    // let wall_color = Color::RGB(150, 150, 150);
    // game_state.entities.add_wall_block(&mut game_state.components, Coordinates { x: 1, y: 2}, Render { color: wall_color })?;
    // game_state.entities.add_wall_block(&mut game_state.components, Coordinates { x: 1, y: 3}, Render { color: wall_color })?;
    // game_state.entities.add_wall_block(&mut game_state.components, Coordinates { x: 1, y: 4}, Render { color: wall_color })?;
    // game_state.entities.add_wall_block(&mut game_state.components, Coordinates { x: 1, y: 5}, Render { color: wall_color })?;

    //game_state.entities.add_timed_square_creator(&mut game_state.components, Coordinates { x: 0, y: 0 }, 50)?;
    game_state.entities.add_timed_square(
        &mut game_state.components, Coordinates { x: 1, y: 1 }, 10, Ai::User, AlignmentType::User, 10,
        Render { color: Color::RGB(100, 100, 100) })?;
    game_state.entities.add_timed_square(
        &mut game_state.components, Coordinates { x: 2, y: 2 }, 10, Ai::AlternateDirections(0, Direction::Left, Direction::Right),
        AlignmentType::Neutral, 2, Render { color: Color::RGB(0, 0, 0) })?;
    game_state.entities.add_timed_square(
        &mut game_state.components, Coordinates { x: 6, y: 4 }, 15, Ai::AlternateDirections(0, Direction::Down, Direction::Up),
        AlignmentType::User, 3, Render { color: Color::RGB(255, 0, 0) })?;
    game_state.entities.add_timed_square(
        &mut game_state.components, Coordinates { x: 8, y: 6 }, 25, Ai::AlternateDirections(0, Direction::Left, Direction::Right),
        AlignmentType::HostileToUser, 4, Render { color: Color::RGB(0, 255, 0) })?;
    game_state.entities.add_timed_square(
        &mut game_state.components, Coordinates { x: 2, y: 8 }, 35, Ai::AlternateDirections(0, Direction::Down, Direction::Up),
        AlignmentType::HostileToUser, 5, Render { color: Color::RGB(0, 0, 255) })?;

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
                Event::KeyDown { keycode: k, .. } => {
                    key_press = k;
                },
                _ => { key_press = None; }
            }
        }
        // The rest of the game loop goes here...
        draw_squares(&game_state.components.coords, game_state.display.coord_scale, &game_state.components.renders, &mut canvas)?;

        if game_state.loop_state == LoopState::RunTimers {
            update_timers(&mut game_state.components.decision_timers, &mut game_state.decisions_ready);
            game_state.loop_state = LoopState::MakeDecisions;
        }

        if game_state.loop_state == LoopState::MakeDecisions {
            let maybe_loop_state = make_decisions(
                &mut game_state.decisions_ready, &mut game_state.components, &mut game_state.planned_actions)?;
            game_state.loop_state = match maybe_loop_state {
                Some(LoopState::User(e_id)) => {
                    println!("Player turn for {e_id}");
                    LoopState::User(e_id)
                },
                Some(x) => x,
                None => LoopState::DoActions
            }
        }

        // Use a match to pull out the e_id.
        match game_state.loop_state {
            LoopState::User(e_id) => match key_press {
                Some(k) => {
                    match make_user_decision(
                        e_id, &k, &mut game_state.planned_actions, &game_state.components)? {
                        Some(l) => game_state.loop_state = l,
                        _ => {}
                    }
                },
                None => {}
            },
            _ => {}
        }

        if game_state.loop_state == LoopState::DoActions {
            do_actions(&mut game_state)?;
            do_reactions(&mut game_state.reactions_ready, &mut game_state.to_kill);
            do_killings(&mut game_state.to_kill, &mut game_state.components, &mut game_state.entities);
            game_state.loop_state = LoopState::RunTimers;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
