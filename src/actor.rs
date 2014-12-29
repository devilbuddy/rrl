use util::{Point, Color, Direction};
use world::{World, ActorRef};

use input;

use std::rand;

pub struct Action {
	move_action: Option<MoveAction>,
	bump_action: Option<BumpAction>
}

struct MoveAction {
	position: Point
}

struct BumpAction {
	position: Point
}

impl Action {
	pub fn make_move_action(positon: &Point) -> Action {
		Action{
			move_action: Some(MoveAction {position: Point::new(positon.x, positon.y)}),
			bump_action: None
		}
	}

	pub fn make_bump_action(positon: &Point) -> Action {
		Action {
			move_action: None,
			bump_action: Some(BumpAction {position: Point::new(positon.x, positon.y)})
		}
	}

	pub fn execute(&self, actor_ref: &ActorRef, world: &mut World) {

		let mut message: Option<String> = None;

		match self.move_action {
			Some(ref move_action) => {
				if world.is_walkable(&move_action.position) {
					world.set_actor_position(actor_ref, &move_action.position);	
				}
			}
			None => {}
		}

		match self.bump_action {
			
			Some(ref bump_action) => {
				let cell = world.get_cell(bump_action.position.x, bump_action.position.y);
				
				match cell.actor {
					Some(ref bump_target_actor_ref) => {
						message = Some(format!("{} bumped by {}", bump_target_actor_ref.borrow().name.as_slice(), actor_ref.borrow().name.as_slice()));
						bump_target_actor_ref.borrow_mut().bumped_by(actor_ref);
					},
					None => { assert!(false) }
				}
				
			},
			None => {}
		}

		// add action message
		match message {
			Some(message) => {
				world.add_message(message.as_slice());
			},
			None => {}
		}
	}
}

pub trait Brain {
	fn think(&self) -> bool;
	fn act(&self, actor_ref: &ActorRef, world: &World) -> Option<Action>;
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

	fn act(&self, actor_ref: &ActorRef, world: &World) -> Option<Action> {
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

        let mut position = Point::new(0,0);
        {
        	let player = actor_ref.borrow();
        	position.x = player.get_position().x;
        	position.y = player.get_position().y;
        }
        position.translate(&direction);

        if world.is_walkable(&position) {
        	Some(Action::make_move_action(&position))	
        } else {
        	let cell = world.get_cell(position.x, position.y);
        	match cell.actor {
        		Some(_) => {
        			return Some(Action::make_bump_action(&position))
        		}
        		None => {}
        	}
        	None
        }
		
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

	fn act(&self, actor_ref: &ActorRef, world: &World) -> Option<Action> {
		let mut direction = Direction::None;
		match rand::random::<uint>()%4 {
			0 => { direction = Direction::North },
			1 => { direction = Direction::South },
			2 => { direction = Direction::East },
			3 => { direction = Direction::West }, 
			_ => { }
		}

		let mut position = Point::new(0,0);
		{
			let actor = actor_ref.borrow();
        	position.x = actor.get_position().x;
        	position.y = actor.get_position().y;
        }
        position.translate(&direction);

        if world.is_walkable(&position) {
        	Some(Action::make_move_action(&position))
        } else {
        	Some(Action::make_move_action(actor_ref.borrow().get_position()))
        }
		
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

	fn act(&self, actor_ref: &ActorRef, world: &World) -> Option<Action> {
		None
	}
}

pub struct Actor {
	pub position: Point,
    pub glyph : char,
    pub color : Color,
    pub name : String,
    pub is_player : bool,
    pub is_solid : bool,
    pub health: uint,
    pub brain : Box<Brain + 'static>
}


impl Actor {
	pub fn player() -> Actor {
		Actor {
			position: Point::new(0,0), 
			glyph: '@', 
			color: Color::red(), 
			name: "player".to_string(),
			is_player: true, 
			is_solid : true, 
			health: 0, 
			brain: box PlayerBrain::new()
		}
	}

	pub fn kobold() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'k', color: Color::green(), name: "kobold".to_string(), is_player: false, is_solid : true, health: 0, brain: box MonsterBrain::new()}
	}

	// ´æøłĸ ŋđðßª
	pub fn kobold_generator() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'O', color: Color::purple(), name: "generator".to_string(),  is_player: false, is_solid : true, health: 0, brain: box NoBrain::new()}	
	}

	pub fn ammo_crate() -> Actor {
		Actor {position: Point::new(0,0), glyph: '*', color: Color::brown(), name: "ammo crate".to_string(), is_player: false, is_solid : false, health: 0, brain: box NoBrain::new()}	
	}

	pub fn get_position(&self) -> &Point {
		return &self.position;
	}

	pub fn set_position(&mut self, position: Point) {
		self.position.x = position.x;
		self.position.y = position.y;
	}

	pub fn bumped_by(&mut self, actor_ref: &ActorRef) {
		self.health -= 1;
	}
}
