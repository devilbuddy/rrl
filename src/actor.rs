use util::Point;

pub struct Actor {
    pub position: Point,
    pub glyph : char
}

impl Actor {
	pub fn new(glyph : char) -> Actor {
		Actor { position: Point::new(0,0), glyph: glyph }
	}	
}
