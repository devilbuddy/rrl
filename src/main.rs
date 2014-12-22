extern crate tcod;
use tcod::{Console};

mod util;
mod actor;
mod world;
mod renderer;
mod input;

fn main() {

	let w = 80;
	let h = 50;

	let player = actor::Actor::new('@');
	let mut world = world::World::new(w, h);
	world.generate();

	let mut renderer = renderer::Renderer::new(w, h);
  
    while !Console::window_closed() {

    	renderer.draw_world(&world);
		renderer.draw_actor(&player);
		renderer.flush();
        
        let key_code = input::wait_for_keypress();
        match key_code {
        	input::KeyCode::Up => {println!("UP")},
        	_ => {}
        }
        //println!("Pressed key: {}", key_code);
    }
}