extern crate core;

use util::Point;
use actor::Actor;

use std::cell::{RefCell};
use std::rc::{Rc};
use std::collections::RingBuf;

type ActorRef = Rc<RefCell<Actor>>;

#[deriving(PartialEq)]
pub enum CellType {
    Wall,
    Floor
}

struct Cell {
	pub cell_type: CellType,
	actor: Option<ActorRef>
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
	pub kills : uint
}
pub struct World {
	pub width: uint,
	pub height: uint,
	pub grid: Vec<Vec<Cell>>,
	pub actors: Vec<ActorRef>,
	pub player: Box<ActorRef>,
	pub player_state : Box<PlayerState>,
	to_act: RingBuf<ActorRef>
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
		let player_state = PlayerState {ammo: 0, kills: 0};
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
				to_act: RingBuf::new() 
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

		let actor_ref_option = self.to_act.pop_front();
		match actor_ref_option {
			Some(actor_ref) => {
				let action = actor_ref.borrow().brain.act();
		 		match action {
		 			Some(move_action) => {
		 				let mut new_position = Point{x: 0, y: 0};
		 				{
		 					let actor = actor_ref.borrow();
							new_position.x = actor.get_position().x;
							new_position.y = actor.get_position().y;
						}
						new_position.translate(move_action.direction);
						
	    				if self.is_walkable(&new_position) { 
	    					self.set_actor_position(&actor_ref, &new_position);
	    				}
		 			},
		 			None => {
		 				// no action taken (player)
		 				self.to_act.push_front(actor_ref.clone());
		 			}
		 		}
			},
			None => {}
		}
		
	}

	fn set_actor_position(&mut self, actor_ref: &ActorRef, position: &Point) {
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

	pub fn is_valid(&self, p: &Point) -> bool {
		return p.x < self.width && p.y < self.height;
	}

	pub fn is_walkable(&self, p: &Point) -> bool {
		return self.is_valid(p) && self.get_cell(p.x, p.y).is_walkable();
	}

	pub fn get_cell(&self, x: uint, y: uint) -> &Cell {
		&self.grid[y][x]
	}

}