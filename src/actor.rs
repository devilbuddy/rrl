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
}
