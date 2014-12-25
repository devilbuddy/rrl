extern crate tcod;
use tcod::{Console, BackgroundFlag};

use actor::Actor;
use world::World;
use util;

pub struct Panel {
	width: uint,
	height: uint,
    con: Console
}

impl Panel {
	pub fn new(x: int, y: int, width: uint, height: uint, background_color: util::Color, foreground_color: util::Color) -> Panel {
		let mut con = Console::new(width as int, height as int);
		con.set_default_background(background_color.to_tcod_color());
		con.set_default_foreground(foreground_color.to_tcod_color());
		con.clear();
		Panel {width: width, height: height, con: con}
	}
}

pub struct Renderer {
    con: Console,
    top_panel: Panel
}

impl Renderer {

	pub fn new(width: uint, height: uint, title: &str) -> Renderer {
		let top_panel_height = 3;
		let top_panel = Panel::new(0, 0, width, top_panel_height, util::Color::blue(), util::Color::black());

		let window_height = height + top_panel_height;
		Renderer {con: Console::init_root(width as int, window_height as int, title, false),
				  top_panel: top_panel }
	}

	pub fn draw_world(&mut self, world: &World) {
		Console::blit(&self.top_panel.con, // source console
						0, 0, self.top_panel.width as int, self.top_panel.height as int, // source 
						&mut self.con, // dest console 
						0, 0, 
						1f32, 
						1f32);

		let y_offset = self.top_panel.height;

		for y in range (0, world.height) {
			for x in range(0, world.width) {
				let cell = world.get_cell(x, y);
				let dest_x = x as int;
				let dest_y = (y + y_offset) as int;
				self.con.put_char(dest_x, dest_y, cell.get_glyph(), BackgroundFlag::Set);		
			}
		}
		
		
	}

	pub fn draw_actor(&mut self, actor: &Actor) {
		let y_offset = self.top_panel.height;
		let dest_y = (actor.position.y + y_offset) as int;
		self.con.put_char_ex(actor.position.x as int, dest_y, actor.glyph, actor.color.to_tcod_color(), util::Color::black().to_tcod_color());
	}
	
	pub fn flush(&self) {
		Console::flush();
	}

}