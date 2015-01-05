use util::{Point, Direction};
use world::{World, ActorRef};
use actor::Actor;

pub struct Action {
	wait_action: Option<WaitAction>,
	spawn_action: Option<SpawnAction>,
	move_action: Option<MoveAction>,
	bump_action: Option<BumpAction>,
	fire_action: Option<FireAction>
}

struct WaitAction;

struct SpawnAction {
	position: Point
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

	pub fn make_wait_action() -> Action {
		Action {
			wait_action: Some(WaitAction),
			spawn_action: None,
			move_action: None,
			bump_action: None,
			fire_action: None
		}		
	}

	pub fn make_spawn_action(position: &Point) -> Action {
		Action {
			wait_action: None,
			spawn_action: Some(SpawnAction {position: Point::new(position.x, position.y)}),
			move_action: None,
			bump_action: None,
			fire_action: None
		}
	}

	pub fn make_move_action(position: &Point) -> Action {
		Action {
			wait_action: None,
			spawn_action: None,
			move_action: Some(MoveAction {position: Point::new(position.x, position.y)}),
			bump_action: None,
			fire_action: None
		}
	}

	pub fn make_bump_action(position: &Point) -> Action {
		Action {
			wait_action: None,
			spawn_action: None,
			move_action: None,
			bump_action: Some(BumpAction {position: Point::new(position.x, position.y)}),
			fire_action: None
		}
	}

	pub fn make_fire_action(direction: Direction) -> Action {
		Action {
			wait_action: None,
			spawn_action: None,
			move_action: None,
			bump_action: None,
			fire_action: Some(FireAction {direction: direction})
		}
	}

	pub fn execute(&self, actor_ref: &ActorRef, world: &mut World) {

		let mut message: Option<String> = None;

		// move
		if let Some(ref move_action) = self.move_action {
			let mut picked_up_item = false;
			{
				let cell = world.get_cell(move_action.position.x, move_action.position.y);
				if let Some(ref item_actor_ref) = cell.actor {
					let mut target = item_actor_ref.borrow_mut();
					target.walked_on_by(actor_ref);
					let pick_up_message = format!("Picked up {}", target.name);
					message = Some(pick_up_message);
					picked_up_item = true;
				}	
			}

			if picked_up_item {
				world.remove_actor(&move_action.position);
			}

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
		

		// fire 
		if let Some(ref fire_action) = self.fire_action {
			let mut target_died = false;
			let mut bullet_position = Point::new(0,0);
			{
				let actor = actor_ref.borrow();
				bullet_position.set(actor.get_position());
				bullet_position.translate(&fire_action.direction);
			}

			loop {
				let mut done = false;

				if world.is_valid(&bullet_position) {
					let cell = world.get_cell(bullet_position.x, bullet_position.y);
					if let Some(ref hit_actor_ref) = cell.actor {
						let mut target = hit_actor_ref.borrow_mut();
						let mut msg_string = format!("{} fired at {}", actor_ref.borrow().name.as_slice(), target.name.as_slice()); 
						
						target.bumped_by(actor_ref);
						target_died = !target.is_alive();

						if target_died {
							let die_message = format!(" - {} dies",  target.name.as_slice());
							msg_string.push_str(die_message.as_slice());
						}

						message = Some(msg_string);
						done = true;
					}	
				} else {
					done = true;
				}
				
				if done {
					break;
				} else {
					bullet_position.translate(&fire_action.direction);
				}
			}

			if target_died {
				world.remove_actor(&bullet_position);
			}

			
		}

		// spawn
		if let Some(ref spawn_action) = self.spawn_action {
			world.add_actor(Actor::kobold(), Point::new(spawn_action.position.x, spawn_action.position.y));
		}

		// wait
		if let Some(_) = self.wait_action {
			// noop
		}
 
		// add action message
		if let Some(message) = message {
    		world.add_message(message.as_slice());
		}
	}
}