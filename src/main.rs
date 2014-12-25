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

	let mut player = actor::Actor::player();
	let mut world = world::World::new(w, h);
	world.generate();
	player.position.set(util::Point {x: world.start.x, y: world.start.y});

	let mut renderer = renderer::Renderer::new(w, h, "rust-rl");
  
    while !Console::window_closed() {

    	renderer.draw_world(&world);
		renderer.draw_actor(&player);
		renderer.flush();
        
        let key_code = input::wait_for_keypress();
        let mut direction = util::Direction::None;


        match key_code {
        	input::KeyCode::Up => { direction = util::Direction::North },
        	input::KeyCode::Down => { direction = util::Direction::South },
        	input::KeyCode::Left => { direction = util::Direction::West },
        	input::KeyCode::Right => { direction = util::Direction::East },
        	input::KeyCode::Escape => { 
        		world.generate();
        		player.position.set(util::Point {x: world.start.x, y: world.start.y});
        	},
        	_ => {}
        }

        // move
		let mut p = util::Point::new(player.position.x, player.position.y);
		p.translate(direction);
        if world.is_walkable(&p) { player.position.set(p) }

        //println!("Pressed key: {}", key_code);
    }
}