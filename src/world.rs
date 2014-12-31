extern crate core;

use util::Point;
use actor::Actor;

use std::cell::{RefCell};
use std::rc::{Rc};
use std::collections::RingBuf;

pub type ActorRef = Rc<RefCell<Actor>>;

#[deriving(PartialEq)]
pub enum CellType {
    Wall,
    Floor
}

struct Cell {
	pub cell_type: CellType,
	pub actor: Option<ActorRef>
}

impl Cell {
	fn new(cell_type: CellType) -> Cell {
		Cell {cell_type: cell_type, actor: None}
	}

	pub fn get_glyph(&self) -> char {
		match self.cell_type {
			CellType::Wall => '#',
			CellType::Floor => '.'
		}
	}

	pub fn is_walkable(&self) -> bool {
		match self.cell_type {
			CellType::Floor => { 
				match self.actor {
					Some(ref actor) => { 
						return !actor.borrow().is_solid; 
					}
					None => { return true; }
				}
			},
			_ => { return false; }
		}
	}
}

pub struct PlayerState {
	pub ammo : uint,
	pub kills : uint,
	pub is_aiming: bool,
}

impl PlayerState {
	pub fn toggle_aiming(&mut self) {
		self.is_aiming = !self.is_aiming;
	}
}

pub struct World {
	pub width: uint,
	pub height: uint,
	pub grid: Vec<Vec<Cell>>,
	pub actors: Vec<ActorRef>,
	pub player: Box<ActorRef>,
	pub player_state : Box<PlayerState>,
	to_act: RingBuf<ActorRef>,
	pub messages : RingBuf<String>
}

impl World {
	pub fn new(width: uint, height: uint) -> World {

		let mut cols:Vec<Vec<Cell>> = Vec::with_capacity(width);

		for _ in range (0, height) {
			let mut rows:Vec<Cell> = Vec::with_capacity(height);
			for _ in range (0, width) {
				rows.push(Cell::new(CellType::Floor));
			}
			cols.push(rows);
		}

		let player = Actor::player();
		let player_state = PlayerState {ammo: 0, kills: 0, is_aiming: false};
		let player_ref = Rc::new(RefCell::new(player));
		let mut actors = Vec::new();
		actors.push(player_ref.clone());

		World {	
				width: width, 
				height: height, 
				grid: cols, 
				actors: actors, 
				player: box player_ref, 
				player_state: box player_state, 
				to_act: RingBuf::new(), 
				messages: RingBuf::new()
			}
	} 

	pub fn tick(&mut self) {

		if self.to_act.is_empty() {
			for actor_ref in self.actors.iter_mut() {
				let actor = actor_ref.borrow();
			 	let can_act = actor.brain.think();
			 	if can_act {
			 		self.to_act.push_back(actor_ref.clone());
			 	}
			}
		}

		if let Some(actor_ref) = self.to_act.pop_front() {
			let mut action_option;
			{
				let actor = actor_ref.borrow();
				if actor.is_alive() {
					action_option = actor.brain.act(&actor_ref, self);	
				} else {
					action_option = None
				}
				
			}
	 		match action_option {
	 			Some(action) => {
	 				action.execute(&actor_ref, self);
	 			},
	 			None => {
	 				// no action taken (player). check again next tick
	 				if(actor_ref.borrow().is_alive()) {
	 					self.to_act.push_front(actor_ref.clone());	
	 				}
	 			}
	 		}
		}
		
		// clean up dead actors
		self.actors.retain(|ref actor_ref| {
			actor_ref.borrow().is_alive()
		});
		
	}

	pub fn set_actor_position(&mut self, actor_ref: &ActorRef, position: &Point) {
		let mut actor = actor_ref.borrow_mut();
		{
			let p = actor.get_position();
			let current_position = Point::new(p.x, p.y);
			
			self.grid[current_position.y][current_position.x].actor = None;
			self.grid[position.y][position.x].actor = Some(actor_ref.clone());
		}
		
		// set new location
		let new_position = Point::new(position.x, position.y);
		actor.deref_mut().set_position(new_position);
	}

	pub fn add_actor(&mut self, actor: Actor, position: Point) {
		let actor_ref = Rc::new(RefCell::new(actor));
		self.set_actor_position(&actor_ref, &position);
		self.actors.push(actor_ref.clone());
	}

	pub fn remove_actor(&mut self, position: &Point) {
		self.grid[position.y][position.x].actor = None;
	}

	pub fn is_valid(&self, p: &Point) -> bool {
		return p.x < self.width && p.y < self.height;
	}

	pub fn is_walkable(&self, p: &Point) -> bool {
		return self.is_valid(p) && self.get_cell(p.x, p.y).is_walkable();
	}

	pub fn get_cell(&self, x: uint, y: uint) -> &Cell {
		&self.grid[y][x]
	}


	pub fn add_message(&mut self, message: &str) {
		self.messages.push_back(String::from_str(message));
		if self.messages.len() > 3 {
			self.messages.pop_front();
		}
	}

}