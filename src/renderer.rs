extern crate tcod;
use tcod::{Console, BackgroundFlag, TextAlignment};

use actor::Actor;
use world::World;
use util;

pub struct Panel {
	x: uint,
	y: uint,
	width: uint,
	height: uint,
    con: Console
}

impl Panel {
	pub fn new(x: uint, y: uint, width: uint, height: uint, background_color: util::Color, foreground_color: util::Color) -> Panel {
		let mut con = Console::new(width as int, height as int);
		con.set_default_background(background_color.to_tcod_color());
		con.set_default_foreground(foreground_color.to_tcod_color());
		con.clear();
		Panel {x: x, y: y, width: width, height: height, con: con}
	}

	pub fn clear(&mut self) {
		self.con.clear();
	} 
}

pub struct Renderer {
	width: uint,
    con: Console,
    top_panel: Panel,
    bottom_panel: Panel
}

impl Renderer {

	pub fn new(width: uint, height: uint, title: &str) -> Renderer {
		let panel_height = 3;
		let top_panel = Panel::new(0, 0, width, panel_height, util::Color::panel_green(), util::Color::black());
		let bottom_panel = Panel::new(0, 0, width, panel_height + height, util::Color::panel_green(), util::Color::black());

		let window_height = height + panel_height + panel_height;
		Renderer {
			width: width,
			con: Console::init_root(width as int, window_height as int, title, false),
			top_panel: top_panel, 
			bottom_panel: bottom_panel
		}
	}

	pub fn draw_title(&mut self) {
		self.con.clear();

		let mut y = 5;
		let x = self.width as int / 2;
		let alignment = TextAlignment::Center;

		self.con.print_ex(x, y, BackgroundFlag::None, alignment,		"#    #  ####  #####   ####  #      #####  "); 
		self.con.print_ex(x, y + 1, BackgroundFlag::None, alignment, 	"#   #  #    # #    # #    # #      #    # ");
		self.con.print_ex(x, y + 2, BackgroundFlag::None, alignment, 	"####   #    # #####  #    # #      #    # ");
		self.con.print_ex(x, y + 3, BackgroundFlag::None, alignment, 	"#  #   #    # #    # #    # #      #    # ");
		self.con.print_ex(x, y + 4, BackgroundFlag::None, alignment, 	"#   #  #    # #    # #    # #      #    # ");
		self.con.print_ex(x, y + 5, BackgroundFlag::None, alignment, 	"#    #  ####  #####   ####  ###### #####  ");
		self.con.print_ex(x, y + 6, BackgroundFlag::None, alignment, 	"                                          ");
		self.con.print_ex(x, y + 7, BackgroundFlag::None, alignment, 	"#    #   ##   #   # #    # ###### #    #  ");
		self.con.print_ex(x, y + 8, BackgroundFlag::None, alignment, 	"##  ##  #  #   # #  #    # #      ##  ##  ");
		self.con.print_ex(x, y + 9, BackgroundFlag::None, alignment, 	"# ## # #    #   #   ###### #####  # ## #  ");
		self.con.print_ex(x, y + 10, BackgroundFlag::None, alignment, 	"#    # ######   #   #    # #      #    #  ");
		self.con.print_ex(x, y + 11, BackgroundFlag::None, alignment, 	"#    # #    #   #   #    # #      #    #  ");
		self.con.print_ex(x, y + 12, BackgroundFlag::None, alignment, 	"#    # #    #   #   #    # ###### #    #  ");
	
		y += 15;
		self.con.print_ex(x, y, BackgroundFlag::None, alignment, 	"Arrow keys to move/fire");
		self.con.print_ex(x, y + 1, BackgroundFlag::None, alignment, 	"Shift - toggle walk/aim");

		y += 4;

		self.draw_title_actor_description(30, y, Actor::player());
		self.draw_title_actor_description(30, y + 1, Actor::kobold());
		self.draw_title_actor_description(30, y + 2, Actor::kobold_generator());
		self.draw_title_actor_description(30, y + 3, Actor::ammo_crate());

		y += 6;
		self.con.print_ex(x, y, BackgroundFlag::None, alignment, 	"[ Press any key to start ]");

		self.flush();
	}

	fn draw_title_actor_description(&mut self, x: int, y: int, actor: Actor) {
		self.con.put_char_ex(x, y, actor.glyph, actor.color.to_tcod_color(), util::Color::black().to_tcod_color());
		self.con.print_ex(x + 2, y , BackgroundFlag::None, TextAlignment::Left, 	actor.name.as_slice());

	}

	pub fn draw_world(&mut self, world: &World) {
		self.draw_top_panel(world);
		self.draw_bottom_panel(world);

		let y_offset = self.top_panel.height;

		for y in range (0, world.height) {
			for x in range(0, world.width) {
				let cell = world.get_cell(x, y);
				let dest_x = x as int;
				let dest_y = (y + y_offset) as int;
				self.con.put_char(dest_x, dest_y, cell.get_glyph(), BackgroundFlag::Set);		
			}
		}
		
		for actor_ref in world.actors.iter() {
			self.draw_actor(actor_ref.borrow().deref());
		}

		self.flush();
	}

	fn draw_top_panel(&mut self, world: &World) {

		self.top_panel.clear();

		let mut y = 0;
		for message in world.messages.iter() {
			self.top_panel.con.print_ex(1, y, BackgroundFlag::None, TextAlignment::Left, message.as_slice());
			y += 1;
		}
		
		Console::blit(&self.top_panel.con, // source console
						self.top_panel.x as int, 
						self.top_panel.y as int, 
						self.top_panel.width as int, 
						self.top_panel.height as int, // source 
						&mut self.con, // dest console 
						0, 0, 
						1f32, 
						1f32);
	}

	fn draw_bottom_panel(&mut self, world: &World) {

		let player = world.player.borrow();
		let player_state = &world.player_state;

		self.bottom_panel.con.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left, "Health:");
		self.bottom_panel.con.print_ex(16, 1, BackgroundFlag::None, TextAlignment::Left, "Ammo:");
		self.bottom_panel.con.print_ex(30, 1, BackgroundFlag::None, TextAlignment::Left, "Kills:");

		if player_state.is_aiming {
			self.bottom_panel.con.print_ex(40, 1, BackgroundFlag::None, TextAlignment::Left, "[Amiming]");
		} else {
			self.bottom_panel.con.print_ex(40, 1, BackgroundFlag::None, TextAlignment::Left, "[Walking]");
		}

		self.bottom_panel.con.print_ex(9, 1, BackgroundFlag::None, TextAlignment::Left, player.health.to_string().as_slice());
		self.bottom_panel.con.print_ex(22, 1, BackgroundFlag::None, TextAlignment::Left, player_state.ammo.to_string().as_slice());
		self.bottom_panel.con.print_ex(37, 1, BackgroundFlag::None, TextAlignment::Left, player_state.kills.to_string().as_slice());

		Console::blit(&self.bottom_panel.con, // source console
						self.bottom_panel.x as int, 
						self.bottom_panel.y as int , 
						self.bottom_panel.width as int, 
						self.bottom_panel.height as int, // source 
						&mut self.con, // dest console 
						0, 50 + self.top_panel.height as int, 
						1f32, 
						1f32);
	}

	fn draw_actor(&mut self, actor: &Actor) {
		let y_offset = self.top_panel.height;
		let dest_y = (actor.get_position().y + y_offset) as int;
		self.con.put_char_ex(actor.get_position().x as int, dest_y, actor.glyph, actor.color.to_tcod_color(), util::Color::black().to_tcod_color());		
	}
	
	pub fn flush(&self) {
		Console::flush();
	}

}