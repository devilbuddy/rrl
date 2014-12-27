extern crate tcod;

pub enum Direction {
	North,
	East,
	South,
	West,
	None
}

pub struct Point {
    pub x: uint,
    pub y: uint
}

impl Point {
	pub fn new(x: uint, y: uint) -> Point {
		Point {x: x, y: y}
	}

	pub fn translate(&mut self, direction: Direction) {
		match direction {
			Direction::North => { self.y -= 1 },
			Direction::East => { self.x += 1 },
			Direction::South => { self.y += 1 },
			Direction::West => { self.x -= 1 },
			_ => {}
		}
	}

}	

pub struct Color {
	r: u8,
	g: u8,
	b: u8
}

impl Color {
	pub fn red() -> Color { Color {r:255, g: 0, b: 0} }
	pub fn green() -> Color { Color {r:0, g: 255, b: 0} }
	pub fn blue() -> Color { Color {r:0, g: 0, b: 255} }
	pub fn black() -> Color { Color {r:0, g: 0, b: 0} }
	pub fn purple() -> Color { Color {r:160, g: 32, b: 240} }
	pub fn to_tcod_color(&self) -> tcod::Color {
		tcod::Color::new(self.r, self.g, self.b)
	}
}
