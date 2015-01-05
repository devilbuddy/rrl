extern crate tcod;

use std::rand;
use std::num::SignedInt;

pub enum Direction {
	North,
	East,
	South,
	West
}

static DIRECTIONS: [Direction, ..4] = [Direction::North, Direction::South, Direction::East, Direction::West];

impl Direction {
	pub fn random_direction() -> Direction {
		match rand::random::<uint>()%4 {
			0 => { Direction::North },
			1 => { Direction::South },
			2 => { Direction::East },
			3 => { Direction::West }, 
			_ => { panic!() }
		}
	}	
}

pub struct Point {
    pub x: uint,
    pub y: uint
}

impl Point {
	pub fn new(x: uint, y: uint) -> Point {
		Point {x: x, y: y}
	}

	pub fn set(&mut self, other: &Point) {
		self.x = other.x;
		self.y = other.y;
	}

	pub fn translate(&mut self, direction: &Direction) {
		match *direction {
			Direction::North => { self.y -= 1 },
			Direction::East => { self.x += 1 },
			Direction::South => { self.y += 1 },
			Direction::West => { self.x -= 1 },
		}
	}

	pub fn distance_to(&self, other: &Point) -> uint {
		let dx = (self.x - other.x) as int;
		let dy = (self.y - other.y) as int;
		return (dx.abs() + dy.abs()) as uint;
	}

	pub fn is_adjacent_to(&self, other: &Point) -> bool {
		for dir in DIRECTIONS.iter() {
		    let mut p = Point::new(self.x, self.y);
		    p.translate(dir);
		    if p.x == other.x && p.y == other.y {
		    	return true;
		    }
		}
		return false;
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
	pub fn black() -> Color { Color {r:0, g: 0, b: 0} }
	pub fn purple() -> Color { Color {r:160, g: 32, b: 240} }
	pub fn panel_green() -> Color { Color {r:76, g:153, b:0} }
	pub fn light_blue() -> Color { Color {r:0, g:102, b:204} }
	pub fn white() -> Color { Color {r:255, g:255, b:255} }

	pub fn to_tcod_color(&self) -> tcod::Color {
		tcod::Color::new(self.r, self.g, self.b)
	}
}
