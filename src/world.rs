extern crate core;

use util::Point;
use std::rand;
use std::num::SignedInt;

#[deriving(PartialEq)]
enum CellType {
    Wall,
    Floor
}

struct Cell {
	cell_type : CellType,
}

impl Cell {
	fn new(cell_type: CellType) -> Cell {
		Cell {cell_type: cell_type}
	}

	pub fn get_glyph(&self) -> char {
		match self.cell_type {
			CellType::Wall => '#',
			CellType::Floor => '.'
		}
	}

	pub fn is_walkable(&self) -> bool {
		match self.cell_type {
			CellType::Floor => { return true; },
			_ => { return false; }
		}
	}
}

pub struct World {
	pub width: uint,
	pub height: uint,
	grid: Vec<Vec<Cell>>,
	pub start: Point
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

		World {width: width, height: height, grid: cols, start: Point::new(0,0)}
	} 

	pub fn generate(&mut self) {
		// http://www.roguebasin.com/index.php?title=Cellular_Automata_Method_for_Generating_Random_Cave-Like_Levels#C_Code

		let fill_prob = 40;
		let generations = 5u;
		let r1_cutoff = 5u;
		let r2_cutoff = 2u;

		let mut grid : Vec<Vec<uint>> = Vec::with_capacity(self.height);
		let mut grid2 : Vec<Vec<uint>> = Vec::with_capacity(self.height);

		// fill grid with random
		for y in range (0, self.height) {
			let mut rows:Vec<uint> = Vec::with_capacity(self.width);
			let mut rows2:Vec<uint> = Vec::with_capacity(self.width);

			for x in range (0, self.width) {
				let mut cell_type = 0;
				if x == 0 || y == 0 || x == self.width - 1 || y == self.height -1 {
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
			for y in range (1i, self.height as int - 1) {
				for x in range (1i, self.width as int - 1) {
					
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
							
							if xx >= 0 && xx < self.width as int && yy >= 0 && yy < self.height as int {
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

			for y in range (0, self.height) {
				for x in range (0, self.width) {
					grid[y][x] = grid2[y][x];
				}
			}
		}

		// all floor tiles
		let mut floors : Vec<Point> = Vec::new();

		for y in range (0, self.height) {
			for x in range (0, self.width) {
				let cell = self.grid.index_mut(&y).index_mut(&x);
				match grid[y][x] {
					1 => { cell.cell_type = CellType::Wall },
					_ => { cell.cell_type = CellType::Floor; floors.push(Point::new(x,y)) },
				}	
			}
		}
		
		// random start positon
		let index = rand::random::<uint>() % floors.len();
		self.start.x = floors[index].x;
		self.start.y = floors[index].y;
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