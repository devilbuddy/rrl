use util::Point;
use util::Color;

pub struct Actor {
    pub position: Point,
    pub glyph : char,
    pub color : Color
}

impl Actor {
	pub fn new(glyph : char, color: Color) -> Actor {
		Actor { position: Point::new(0,0), glyph: glyph, color: color }
	}

	pub fn set_position(&mut self, position: Point) {
		self.position.x = position.x;
		self.position.y = position.y;
	}
}
