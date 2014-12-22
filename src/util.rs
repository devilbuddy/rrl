pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
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
			Direction::N => { self.y -= 1 },
			Direction::NE => { self.x += 1; self.y -=1 },
			Direction::E => { self.x += 1 },
			Direction::SE => { self.x += 1; self.y += 1 },
			Direction::S => { self.y += 1 },
			Direction::SW => { self.x -= 1; self.y += 1 },
			Direction::W => { self.x -= 1 },
			Direction::NW => { self.x -= 1; self.y -= 1 },
			_ => {}
		}
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
