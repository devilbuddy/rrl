extern crate tcod;

pub enum Direction {
	North,
	NorthEast,
	East,
	SouthEast,
	South,
	SouthWest,
	West,
	NorthWest,

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
			Direction::NorthEast => { self.x += 1; self.y -=1 },
			Direction::East => { self.x += 1 },
			Direction::SouthEast => { self.x += 1; self.y += 1 },
			Direction::South => { self.y += 1 },
			Direction::SouthWest => { self.x -= 1; self.y += 1 },
			Direction::West => { self.x -= 1 },
			Direction::NorthWest => { self.x -= 1; self.y -= 1 },
			_ => {}
		}
	}

	pub fn set(&mut self, p : Point) {
		self.x = p.x;
		self.y = p.y;
	}
}	

pub struct Rect {
    pub x: uint,
    pub y: uint,
    pub width: uint,
    pub height: uint,
}

impl Rect {
	pub fn new(x: uint, y: uint, width: uint, height:uint) -> Rect {
		Rect {x: x, y: y, width: width, height: height}
	}
}

pub struct Color {
	r: u8,
	g: u8,
	b: u8
}

impl Color {
	pub fn red() -> Color { Color {r:255, g: 0, b: 0} }
	pub fn black() -> Color { Color {r:0, g: 0, b: 0} }

	pub fn to_tcod_color(&self) -> tcod::Color {
		tcod::Color::new(self.r, self.g, self.b)
	}
}
