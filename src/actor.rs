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
		println!("player think");
		return true;
	}

	fn act(&self) -> Option<MoveAction> {
		println!("before wait");
		let key_code = input::wait_for_keypress();
		println!("after wait");
        let mut direction = Direction::None;
		match key_code {
        	input::KeyCode::Up => { direction = Direction::North },
        	input::KeyCode::Down => { direction = Direction::South },
        	input::KeyCode::Left => { direction = Direction::West },
        	input::KeyCode::Right => { direction = Direction::East },
        	input::KeyCode::Escape => { 
        		//world.generate();
        	},
        	_ => { return None; }
        }

		Some(MoveAction{direction: direction})
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

pub struct Actor {
	pub position: Point,
    pub glyph : char,
    pub color : Color,
    pub is_player : bool,
    pub brain : Box<Brain + 'static>
}


impl Actor {
	pub fn player() -> Actor {
		Actor {position: Point::new(0,0), glyph: '@', color: Color::red(), is_player: true, brain: box PlayerBrain::new()}
	}

	pub fn kobold() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'k', color: Color::green(), is_player: false, brain: box MonsterBrain::new()}
	}

	pub fn get_position(&self) -> &Point {
		return &self.position;
	}

	pub fn set_position(&mut self, position: Point) {
		self.position.x = position.x;
		self.position.y = position.y;
	}

}
