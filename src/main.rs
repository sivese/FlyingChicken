extern crate sdl2;

mod app;
mod core;

use app::game::Game;

/* https://github.com/gameprogcpp/code */
fn main() {
    let mut game = match Game::initialize() {
        Ok(game) => game,
        Err(err) => { panic!("Couldn't initialize SDL system!") }
    };

    game.run_loop();

    unsafe { game.shutdown(); } // raw C function call -> SDL_Quit
}