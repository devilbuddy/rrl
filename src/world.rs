use util::Point;
use std::rand;

enum CellType {
    Wall,
    Floor
}

struct Cell {
	cell_type : CellType,
	position : Point
}

impl Cell {
	fn new(x: uint, y: uint, cell_type: CellType) -> Cell {
		Cell {cell_type: cell_type, position: Point::new(x, y)}
	}

	pub fn get_glyph(&self) -> char {
		match self.cell_type {
			CellType::Wall => '#',
			CellType::Floor => '.'
		}
	}
}

pub struct World {
	pub width: uint,
	pub height: uint,
	grid: Vec<Vec<Cell>>,
}

impl World {
	pub fn new(width: uint, height: uint) -> World {

		let mut cols:Vec<Vec<Cell>> = Vec::with_capacity(width);
		for x in range (0, height) {
			let mut rows:Vec<Cell> = Vec::with_capacity(height);
			for y in range (0, width) {
				rows.push(Cell::new(x, y, CellType::Floor));
			}
			cols.push(rows);
		}


		World {width: width, height: height, grid: cols}
	} 

	pub fn generate(&mut self) {
		for x in range (0, self.height) {
			for y in range (0, self.width) {
				let cell = self.grid.index_mut(&x).index_mut(&y);
				if(rand::random()) {
					cell.cell_type = CellType::Wall;
				} else {
					cell.cell_type = CellType::Floor;
				}
			}
		}
	}

	pub fn get_cell(&self, x: uint, y: uint) -> &Cell {
		&self.grid[y][x]
	}
}