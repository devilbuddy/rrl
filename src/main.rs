extern crate tcod;
use tcod::{Console};

mod util;
mod actor;
mod action;
mod world;
mod generator;
mod renderer;
mod input;

fn main() {

	println!("main");

	let w = 80;
	let h = 50;

	let mut world = world::World::new(w, h);
	generator::generate(&mut world);
	
	let mut renderer = renderer::Renderer::new(w, h, "rust-rl");
  	renderer.draw_world(&world);

    while !Console::window_closed() {
    	world.tick();
    	renderer.draw_world(&world);
    }
}