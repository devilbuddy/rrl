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
					Some(_) => { return false; }
					None => { return true; }
				}
			},
			_ => { return false; }
		}
	}
}


pub struct World {
	pub width: uint,
	pub height: uint,
	pub grid: Vec<Vec<Cell>>,
	pub start: Point,
	pub actors: Vec<ActorRef>,
	pub player: Box<ActorRef>,
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
		let player_ref = Rc::new(RefCell::new(player));

		let mut actors = Vec::new();
		actors.push(player_ref.clone());
		World {width: width, height: height, grid: cols, start: Point::new(0,0), actors: actors, player: box player_ref, to_act: RingBuf::new()}
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
				let mut actor = actor_ref.borrow_mut();
				let action = actor.brain.act();
		 		match action {
		 			Some(move_action) => {

		 				let current_position = Point::new(actor.get_position().x, actor.get_position().y);

        				let mut new_position = Point::new(actor.get_position().x, actor.get_position().y);
						new_position.translate(move_action.direction);
						
						let walkable = self.is_walkable(&new_position);
	    				if walkable { 
	    					
							self.grid[current_position.y][current_position.x].actor = None;
							
							
							self.grid[new_position.y][new_position.x].actor = Some(actor_ref.clone());
							actor.set_position(new_position);
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