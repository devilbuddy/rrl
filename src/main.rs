extern crate tcod;
use tcod::{Console};

mod util;
mod actor;
mod world;
mod renderer;
mod input;


fn main() {

	println!("main");

	let w = 80;
	let h = 50;

	let mut world = world::World::new(w, h);
	world.generate();
	
	let mut renderer = renderer::Renderer::new(w, h, "rust-rl");
  	renderer.draw_world(&world);

    while !Console::window_closed() {

    	world.tick();
    	renderer.draw_world(&world);
    }
}