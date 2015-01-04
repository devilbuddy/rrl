use util::{Point, Color, Direction};
use world::{World, ActorRef};
use input;
use action::Action;

use std::rand;

pub trait Brain {
	fn think(&self) -> bool;
	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action>;
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

	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action> {
		let mut direction;
		match input::check_for_keypress() {
			Some(key_code) => {
				match key_code {
					input::KeyCode::Up => { direction = Direction::North },
        			input::KeyCode::Down => { direction = Direction::South },
        			input::KeyCode::Left => { direction = Direction::West },
        			input::KeyCode::Right => { direction = Direction::East },
        			input::KeyCode::ToggleAim => { 
        				world.player_state.toggle_aiming();
						return None; 
        			},
        			_ => { return None; }
				}
			},
			None => {
				return None;
			}
        }

        if world.player_state.is_aiming {
        	// fire
        	return Some(Action::make_fire_action(direction));
        } else {
        	// walk
	        let mut position = Point::new(0,0);
	        {
	        	position.set(actor_ref.borrow().get_position());
	        }
	        position.translate(&direction);

	        if world.is_walkable(&position) {
	        	Some(Action::make_move_action(&position))	
	        } else {
				let cell = world.get_cell(position.x, position.y);
	        	if let Some(_) = cell.actor {
	        		return Some(Action::make_bump_action(&position))
	        	}
	        	None
	        }	
        }
        
		
	}
}

enum MonsterState {
	Passive,
	Aggressive
}

struct MonsterBrain {
	state: MonsterState
}

impl MonsterBrain {
	pub fn new() -> MonsterBrain {
		MonsterBrain{state: MonsterState::Passive}
	}
}

impl Brain for MonsterBrain {
	fn think(&self) -> bool {
		return true;
	}

	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action> {

		match self.state {
			MonsterState::Passive => {

			}
			MonsterState::Aggressive => {

			}
		}

		let distance_to_player =  actor_ref.borrow().position.distance_to(&world.get_player_position());
		if distance_to_player < 10 {
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
	        	position.set(actor_ref.borrow().get_position());
	        }
	        position.translate(&direction);

	        if world.is_walkable(&position) {
	        	return Some(Action::make_move_action(&position));	
	        } else {
	        	let cell = world.get_cell(position.x, position.y);
	        	if let Some(ref actor_ref) = cell.actor {
	        		if actor_ref.borrow().is_player {
	        			return Some(Action::make_bump_action(&position))	
	        		}
	        	} 
	        	
	        }
		}
		
        return Some(Action::make_wait_action());
	}
}

struct GeneratorBrain;
impl GeneratorBrain {
	pub fn new() -> GeneratorBrain {
		GeneratorBrain
	}
}

impl Brain for GeneratorBrain {
	fn think(&self) -> bool {
		if rand::random::<uint>()%10 == 1 {
			return true;
		}
		return false;
	}

	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action> {
		let mut direction = Direction::None;
		match rand::random::<uint>()%4 {
			0 => { direction = Direction::North },
			1 => { direction = Direction::South },
			2 => { direction = Direction::East },
			3 => { direction = Direction::West }, 
			_ => { }
		}

		let mut spawn_position = Point::new(0,0);
		{
        	spawn_position.set(actor_ref.borrow().get_position());
        }
        spawn_position.translate(&direction);

        if world.is_walkable(&spawn_position) {
        	Some(Action::make_spawn_action(&spawn_position))
        } else {
        	Some(Action::make_wait_action())
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

	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action> {
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
			health: 100, 
			brain: box PlayerBrain::new()
		}
	}

	pub fn kobold() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'k', color: Color::green(), name: "kobold".to_string(), is_player: false, is_solid : true, health: 1, brain: box MonsterBrain::new()}
	}

	pub fn kobold_generator() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'O', color: Color::purple(), name: "generator".to_string(),  is_player: false, is_solid : true, health: 1, brain: box GeneratorBrain::new()}	
	}

	pub fn ammo_crate() -> Actor {
		Actor {position: Point::new(0,0), glyph: '*', color: Color::light_blue(), name: "ammo crate".to_string(), is_player: false, is_solid : false, health: 1, brain: box NoBrain::new()}	
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

	pub fn walked_on_by(&mut self, actor_ref: &ActorRef) {
		self.health = 0;
	}

	pub fn is_alive(&self) -> bool {
		return self.health > 0;
	}
}
