use util::{Point, Color, Direction};
use world::{World, ActorRef};

use input;

use std::rand;

pub struct Action {
	move_action: Option<MoveAction>,
	bump_action: Option<BumpAction>,
	fire_action: Option<FireAction>
}

struct MoveAction {
	position: Point
}

struct BumpAction {
	position: Point
}

struct FireAction {
	direction: Direction
}

impl Action {
	pub fn make_move_action(position: &Point) -> Action {
		Action{
			move_action: Some(MoveAction {position: Point::new(position.x, position.y)}),
			bump_action: None,
			fire_action: None
		}
	}

	pub fn make_bump_action(position: &Point) -> Action {
		Action {
			move_action: None,
			bump_action: Some(BumpAction {position: Point::new(position.x, position.y)}),
			fire_action: None
		}
	}

	pub fn make_fire_action(direction: Direction) -> Action {
		Action {
			move_action: None,
			bump_action: None,
			fire_action: Some(FireAction {direction: direction})
		}
	}

	pub fn execute(&self, actor_ref: &ActorRef, world: &mut World) {

		let mut message: Option<String> = None;

		// move
		if let Some(ref move_action) = self.move_action {
			if world.is_walkable(&move_action.position) {
				world.set_actor_position(actor_ref, &move_action.position);	
			}
		}
		
		// bump
		if let Some(ref bump_action) = self.bump_action {
			let mut target_died = false;
			{
				let cell = world.get_cell(bump_action.position.x, bump_action.position.y);	
				match cell.actor {
					Some(ref bump_target_actor_ref) => {
						let mut target = bump_target_actor_ref.borrow_mut();
						let mut msg_string = format!("{} bumped by {}", target.name.as_slice(), actor_ref.borrow().name.as_slice());
						
						target.bumped_by(actor_ref);
						target_died = !target.is_alive();

						if target_died {
							let die_message = format!(" - {} dies",  target.name.as_slice());
							msg_string.push_str(die_message.as_slice());
						}

						message = Some(msg_string);
					},
					None => { assert!(false) }
				}
			}

			if target_died {

				world.remove_actor(&bump_action.position);
			}
		}
		
		if let Some(ref fire_action) = self.fire_action {
			println!("fire");
			let mut p = Point::new(0,0);
			{
				let actor = actor_ref.borrow();
				p.set(actor.get_position());
				p.translate(&fire_action.direction);
			}

			loop {
				let mut done = false;

				if(world.is_valid(&p)) {
					let cell = world.get_cell(p.x, p.y);
					if let Some(ref hit_actor_ref) = cell.actor {
						let mut target = hit_actor_ref.borrow_mut();
						let mut msg_string = format!("{} fired at {}", actor_ref.borrow().name.as_slice(), target.name.as_slice()); 
						

						message = Some(msg_string);
						done = true;
					}	
				} else {
					done = true;
				}
				
				if done {
					break;
				} else {
					p.translate(&fire_action.direction);
				}
			}
			
		}

		// add action message
		if let Some(message) = message {
    		world.add_message(message.as_slice());
		}
	}
}

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
		let mut direction = Direction::None;
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

	fn act(&self, actor_ref: &ActorRef, world: &mut World) -> Option<Action> {
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
		Actor {position: Point::new(0,0), glyph: 'O', color: Color::purple(), name: "generator".to_string(),  is_player: false, is_solid : true, health: 1, brain: box NoBrain::new()}	
	}

	pub fn ammo_crate() -> Actor {
		Actor {position: Point::new(0,0), glyph: '*', color: Color::brown(), name: "ammo crate".to_string(), is_player: false, is_solid : false, health: 1, brain: box NoBrain::new()}	
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
}
