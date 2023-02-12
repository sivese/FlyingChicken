use sdl2::sys::{ KeyCode, SDL_Quit };
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::{Sdl, event::Event, video::Window };
use sdl2::keyboard::Keycode;

use crate::math::vector::Vector2;

use std::time::Duration;

pub struct Game {
    run : bool,
    canvas : Canvas<Window>,
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

        let canvas_result = window.into_canvas()
        .present_vsync()
        .build();
        
        let mut canvas = match canvas_result {
            Ok(c) => c,
            Err(err) => {
                println!("failed to get canvas. SDL error : {}", err);
                return Result::Err(false);
            }
        };
        
        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        return Result::Ok( Game {
            run : true,
            canvas : canvas,
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
                Event::Quit {..} => 
                {
                    self.run = false;
                    break; // ignore other input events
                }, // event type -> event value matching
                _ => { }
            }
        }
    }
    
    fn update_game(&mut self) {
        let draw_color = Color::RGBA(128, 0, 128, 255);
        
        //self.canvas.set_draw_color(draw_color);
        self.canvas.fill_rect(Rect::new(30, 30, 512, 30 ));

        self.canvas.present();
        //self.canvas.clear();
    }
    
    fn generate_output(&self) { }
}