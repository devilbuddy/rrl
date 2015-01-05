use util::{Point, Color, Direction};
use world::{World, ActorRef};
use input;
use action::Action;

use std::rand;
use std::collections::RingBuf;

pub trait Brain {
	fn think(&self) -> bool;
	fn act(&mut self, current_position: &Point, world: &mut World) -> Option<Action>;
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

	fn act(&mut self, current_position: &Point, world: &mut World) -> Option<Action> {
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
        	if world.has_ammo() {
        		world.decrease_ammo();
        		return Some(Action::make_fire_action(direction));	
        	} else {
        		world.add_message("Out of ammo!");
        		None	
        	}
        	
        } else {
        	// walk
	        let mut position = Point::new(current_position.x, current_position.y);
	        position.translate(&direction);

	        if world.is_walkable(&position) {
	        	return Some(Action::make_move_action(&position));	
	        } else if world.is_bumpable(&position, false) {
				return Some(Action::make_bump_action(&position));
	        }
	        None	
        }
        
		
	}
}

enum MonsterState {
	Passive,
	Aggressive
}

struct MonsterBrain {
	state: MonsterState,
	path: RingBuf<Point>,
	stuck_on_path_count: uint
}

impl MonsterBrain {
	pub fn new() -> MonsterBrain {
		MonsterBrain {
			state: MonsterState::Passive,
			path: RingBuf::new(),
			stuck_on_path_count: 0
		}
	}

	pub fn has_path(&self) -> bool {
		return self.path.len() > 0;
	}

}

impl Brain for MonsterBrain {
	fn think(&self) -> bool {
		return true;
	}

	fn act(&mut self, current_position: &Point, world: &mut World) -> Option<Action> {

		match self.state {
			MonsterState::Passive => {
				//let distance_to_player =  current_position.distance_to(&world.get_player_position());
				//if distance_to_player < 10 {
					self.state = MonsterState::Aggressive;
				//}
			}
			MonsterState::Aggressive => {
				
				if !self.has_path() {
					let from = Point::new(current_position.x, current_position.y);
					let mut to = Point::new(0,0);
					to.set(&world.get_player_position());
					
					if let Some(path) = world.find_path(&from, &to) {
						for p in path.into_iter() {
							self.path.push_back(p);	    
						}
						self.stuck_on_path_count = 0;
					} else {
						self.stuck_on_path_count += 1;
					}
				} 
				
				let mut next = Point::new(0,0);
				if self.has_path() {
					if let Some(p) = self.path.pop_front() {
						next.set(&p);	
					}
				} else if self.stuck_on_path_count > 3 {
					next.set(current_position);
					next.translate(&Direction::random_direction());
				}
				
				if world.is_walkable(&next) {
					return Some(Action::make_move_action(&next));
				} else if world.is_bumpable(&next, true) {
					return Some(Action::make_bump_action(&next));
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
		let should_spawn = rand::random::<uint>()%10 == 1;
		return should_spawn;
	}

	fn act(&mut self, current_position: &Point, world: &mut World) -> Option<Action> {
		let direction = Direction::random_direction();

		let mut spawn_position = Point::new(current_position.x, current_position.y);
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

	#[allow(unused_variables)]
	fn act(&mut self, current_position: &Point, world: &mut World) -> Option<Action> {
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
			name: "Player".to_string(),
			is_player: true, 
			is_solid : true, 
			health: 10, 
			brain: box PlayerBrain::new()
		}
	}

	pub fn kobold() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'k', color: Color::green(), name: "Kobold".to_string(), is_player: false, is_solid : true, health: 1, brain: box MonsterBrain::new()}
	}

	pub fn kobold_generator() -> Actor {
		Actor {position: Point::new(0,0), glyph: 'G', color: Color::purple(), name: "Kobold generator".to_string(),  is_player: false, is_solid : true, health: 1, brain: box GeneratorBrain::new()}	
	}

	pub fn ammo_crate() -> Actor {
		Actor {position: Point::new(0,0), glyph: '*', color: Color::light_blue(), name: "Ammo crate".to_string(), is_player: false, is_solid : false, health: 1, brain: box NoBrain::new()}	
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

	pub fn is_alive(&self) -> bool {
		return self.health > 0;
	}

	pub fn act(&mut self, world: &mut World) -> Option<Action> {
		return self.brain.act(&self.position, world);
	}
}
