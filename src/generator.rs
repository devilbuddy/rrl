
use util::Point;
use actor::Actor;
use world::{World, CellType};

use std::rand;
use std::num::SignedInt;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn generate(world: &mut World) {
		println!("generate");

		// http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels#C_Code

		world.actors.clear();
		world.actors.push(*world.player.clone());

		let fill_prob = 40;
		let generations = 5u;
		let r1_cutoff = 5u;
		let r2_cutoff = 2u;

		let mut grid : Vec<Vec<uint>> = Vec::with_capacity(world.height);
		let mut grid2 : Vec<Vec<uint>> = Vec::with_capacity(world.height);

		// fill grid with random
		for y in range (0, world.height) {
			let mut rows:Vec<uint> = Vec::with_capacity(world.width);
			let mut rows2:Vec<uint> = Vec::with_capacity(world.width);

			for x in range (0, world.width) {
				let mut cell_type = 0;
				if x == 0 || y == 0 || x == world.width - 1 || y == world.height -1 {
					cell_type = 1
				} else if rand::random::<uint>() % 100 < fill_prob {
					cell_type = 1;
				}
				rows.push(cell_type);
				rows2.push(1);
			}
			grid.push(rows);
			grid2.push(rows2);
		}

		for _ in range (0, generations) {
			for y in range (1i, world.height as int - 1) {
				for x in range (1i, world.width as int - 1) {
					
					let mut adjacent_count_r1 = 0u;
					let mut adjacent_count_r2 = 0u;
					
					// the number of tiles within 1 step of p which are walls
					for yy in range (-1i, 2) {
						for xx in range (-1i, 2) {
							let yyy = y + yy;
							let xxx = x + xx;
							if grid[yyy as uint][xxx as uint] == 1 {
							 	adjacent_count_r1 += 1; 
							} 	
						}
					}
					// p is in the middle of an open space
					for yy in range (y - 2i, y + 3) {
						for xx in range (x -2i, x + 3) {
							
							// skip p itself
							if (xx - x).abs() == 2 && (yy - y).abs() == 2 {
								continue;
							}
							
							if xx >= 0 && xx < world.width as int && yy >= 0 && yy < world.height as int {
								if grid[yy as uint][xx as uint] == 1 {
									adjacent_count_r2 += 1;
								}
							}
						}
					}

					if adjacent_count_r1 >= r1_cutoff || adjacent_count_r2 <= r2_cutoff {
						grid2[y as uint][x as uint] = 1;
					} else {
						grid2[y as uint][x as uint] = 0;
					}
				}
			}

			for y in range (0, world.height) {
				for x in range (0, world.width) {
					grid[y][x] = grid2[y][x];
				}
			}
		}

		// all floor tiles
		let mut floors : Vec<Point> = Vec::new();

		for y in range (0, world.height) {
			for x in range (0, world.width) {
				let cell = world.grid.index_mut(&y).index_mut(&x);
				match grid[y][x] {
					1 => { cell.cell_type = CellType::Wall },
					_ => { cell.cell_type = CellType::Floor; floors.push(Point::new(x,y)) },
				}	
			}
		}
		
		// random start positon
		let index = rand::random::<uint>() % floors.len();
		let mut p = world.player.borrow_mut();
		p.set_position(Point {x: floors[index].x, y: floors[index].y});
		floors.remove(index);

		let enemies_count = 10u;
		for _ in range(0, enemies_count) {
			let index = rand::random::<uint>() % floors.len();
			
			let mut monster = Actor::kobold();
			monster.set_position(Point{x: floors[index].x, y: floors[index].y});
			let monster_ref = Rc::new(RefCell::new(monster));
			world.actors.push(monster_ref.clone());

			floors.remove(index);			
		}

		let generators_count = 10u;
		for _ in range(0, generators_count) {
			let index = rand::random::<uint>() % floors.len();
			
			let mut monster = Actor::kobold_generator();
			monster.set_position(Point{x: floors[index].x, y: floors[index].y});
			let monster_ref = Rc::new(RefCell::new(monster));
			world.actors.push(monster_ref.clone());

			floors.remove(index);			
		}



	}