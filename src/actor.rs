use util::Point;
use util::Color;
use util::Direction;

use input;

use std::rand;

pub struct MoveAction {
	pub direction: Direction
}

impl MoveAction {
	pub fn new(direction: Direction) -> MoveAction {
		MoveAction{direction: direction}
	}
}

pub trait Brain {
	fn think(&self) -> bool;
	fn act(&self) -> Option<MoveAction>;
}

struct PlayerBrain;

impl PlayerBrain {
	pub fn new() -> PlayerBrain {
		PlayerBrain
	}
}


impl Brain for PlayerBrain {

	fn think(&self) -> bool {
		return true;
	}

	fn act(&self) -> Option<MoveAction> {
		let mut direction = Direction::None;
		match input::check_for_keypress() {
			Some(key_code) => {
				match key_code {
					input::KeyCode::Up => { direction = Direction::North },
        			input::KeyCode::Down => { direction = Direction::South },
        			input::KeyCode::Left => { direction = Direction::West },
        			input::KeyCode::Right => { direction = Direction::East },
        			_ => {}
				}
			},
			None => {
				return None;
			}
        }

		Some(MoveAction::new(direction))
	}
}


struct MonsterBrain;

impl MonsterBrain {
	pub fn new() -> MonsterBrain {
		MonsterBrain
	}
}

impl Brain for MonsterBrain {
	fn think(&self) -> bool {
		return true;
	}

	fn act(&self) -> Option<MoveAction> {
		let mut direction = Direction::None;
		match rand::random::<uint>()%4 {
			0 => { direction = Direction::North },
			1 => { direction = Direction::South },
			2 => { direction = Direction::East },
			3 => { direction = Direction::West }, 
			_ => { }
		}
		Some(MoveAction{direction: direction})
	}
}

struct NoBrain;
impl NoBrain {
	pub fn new() -> NoBrain {
		NoBrain
	}
}

impl Brain for NoBrain {
	fn think(&self) -> bool {
		return false;
	}

	fn act(&self) -> Option<MoveAction> {
		None
	}
}

pub struct Actor {
	pub position: Point,
    pub glyph : char,
    pub color : Color,
    pub is_player : bool,
    pub is_solid : bool,
    pub health: uint,
    pub brain : Box<Brain + 'static>
}


impl Actor {
	pub fn player() -> Actor {
		Actor {position: Point::new(0,0), glyph: '@', color: Color::red(), is_player: true, is_solid : true, health: 0, brain: box PlayerBrain::new()}
	}

	pub fn kobold() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'k', color: Color::green(), is_player: false, is_solid : true, health: 0, brain: box MonsterBrain::new()}
	}

	pub fn kobold_generator() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'Å', color: Color::purple(), is_player: false, is_solid : true, health: 0, brain: box NoBrain::new()}	
	}

	pub fn ammo_crate() -> Actor {
		Actor {position: Point::new(0,0), glyph: '*', color: Color::brown(), is_player: false, is_solid : false, health: 0, brain: box NoBrain::new()}	
	}

	pub fn get_position(&self) -> &Point {
		return &self.position;
	}

	pub fn set_position(&mut self, position: Point) {
		self.position.x = position.x;
		self.position.y = position.y;
	}

}
