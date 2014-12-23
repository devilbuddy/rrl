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

	let mut player = actor::Actor::new('@', util::Color::red());
	let mut world = world::World::new(w, h);
	world.generate_cellular_automata();
	player.position.x = world.start.x;
	player.position.y = world.start.y;

	let mut renderer = renderer::Renderer::new(w, h, "rust-rl");
  
    while !Console::window_closed() {

    	renderer.draw_world(&world);
		renderer.draw_actor(&player);
		renderer.flush();
        
        let key_code = input::wait_for_keypress();
        let mut direction = util::Direction::None;

        let mut p = util::Point::new(player.position.x, player.position.y);

        match key_code {
        	input::KeyCode::Up => { direction = util::Direction::North },
        	input::KeyCode::Down => { direction = util::Direction::South },
        	input::KeyCode::Left => { direction = util::Direction::West },
        	input::KeyCode::Right => { direction = util::Direction::East },
        	_ => {}
        }
        p.translate(direction);

        if world.is_walkable(&p) { player.position.set(p) }

        //println!("Pressed key: {}", key_code);
    }
}