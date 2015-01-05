extern crate tcod;
use tcod::{Console};

mod util;
mod actor;
mod action;
mod world;
mod generator;
mod renderer;
mod input;

enum State {
	Title,
	Game,
	GameOver
}

fn main() {

	println!("main");

	let mut state = State::Title;

	let w = 80;
	let h = 50;

	let mut renderer = renderer::Renderer::new(w, h, "kobold mayhem");

	let mut world = world::World::new(w, h);
	

    while !Console::window_closed() {
    	match state {
    		State::Title => {
    			renderer.draw_title();
    			input::wait_for_any_key();
    			state = State::Game;

    			// generate and draw world once
    			generator::generate(&mut world);
				renderer.draw_world(&world);
    		},
    		State::Game => {
    			world.tick();
    			renderer.draw_world(&world);
    		}
    		State::GameOver => {

    		}
    	}
    	
    }
}