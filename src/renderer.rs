extern crate tcod;
use tcod::{Console, BackgroundFlag};

use actor::Actor;
use world::World;

pub struct Renderer {
    con: Console
}

impl Renderer {

	pub fn new(width: uint, height: uint) -> Renderer {
		Renderer {con: Console::init_root(width as int, height as int, "rust-rl", false)}
	}

	pub fn draw_world(&mut self, world: &World) {
		for y in range (0, world.height) {
			for x in range(0, world.width) {
				let cell = world.get_cell(x, y);
				self.con.put_char(x as int, y as int, cell.get_glyph(), BackgroundFlag::Set);		
			}
		}
	}

	pub fn draw_actor(&mut self, actor: &Actor) {
		self.con.put_char(actor.position.x as int, actor.position.y as int, actor.glyph, BackgroundFlag::Set);
	}
	
	pub fn flush(&self) {
		Console::flush();
	}
}