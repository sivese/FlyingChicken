use sdl2::sys::KeyCode;
use sdl2::{Sdl, event::Event, video::Window, sys::SDL_Quit};
use sdl2::keyboard::Keycode;

use std::time::Duration;

pub struct Game {
    run : bool,
    window : Window,
    context : Sdl
}

/* public parts */
impl Game {
    pub fn initialize() -> Result<Game, bool> {
        let sdl_context = match sdl2::init() {
            Ok(cxt) => cxt, // cxt = context
            Err(err) => {
                println!("failed to initialize context. error : {}", err);
                return Result::Err(false);
            }
        };

        let video_subsystem = match sdl_context.video() {
            Ok(sub) => sub,
            Err(err) => {
                println!("failed to load video subsystem. error : {}", err);
                return Result::Err(false);
            }
        };

        let window_result = video_subsystem.window("Flying Chicken", 800, 500)
            .position_centered()
            .build();
        
        let window = match window_result {
            Ok(win) => win,
            Err(err) => {
                println!("failed to build window. error : {}", err);
                return Result::Err(false);
            }
        };

        return Result::Ok( Game {
            run : true,
            window : window,
            context : sdl_context
        });
    }

    pub unsafe fn shutdown(&self) { 
        SDL_Quit();
    } // raw C function call

    pub fn run_loop(&mut self) {
        loop {
            if !self.run { break; }

            self.process_input();
            self.update_game();
            self.generate_output();

        }
    }
}

/* private parts */
impl Game {
    fn process_input(&mut self) {
        let mut event_pulled = self.context
            .event_pump()
            .unwrap();

        for event in event_pulled.poll_iter() {
            match event {
                Event::KeyDown { keycode : Some(Keycode::Escape), ..} | // or condition 
                Event::Quit {..} => self.run = false, // event type -> event value matching
                _ => { }
            }
        }
    }

    fn update_game(&self) { }
    fn generate_output(&self) { }
}