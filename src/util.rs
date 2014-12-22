enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW
}

pub struct Point {
    pub x: uint,
    pub y: uint
}

impl Point {
	pub fn new(x: uint, y: uint) -> Point {
		Point {x: x, y: y}
	}
}	

