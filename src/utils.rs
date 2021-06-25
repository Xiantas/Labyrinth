/*
	Fichier de définition d'outils tel que le coloriage
	bicolore 'Coloring' ou encore des objects de direction
	et de point
*/

#![allow(non_snake_case)]

use crate::labyrinth::Labyrinth;

pub struct Coloring {
	grid: Vec<bool>,
	width: usize,
	height: usize
}

impl Coloring {
	pub fn new(labyrinth: &Labyrinth) -> Coloring {
		let width = labyrinth.width();
		let height = labyrinth.height();

		Coloring {
			grid: vec![false; width*height],
			width,
			height
		}
	}

	pub fn set(&mut self, point: Point, val: bool) {
		if point.0 < self.width && point.1 < self.height {
			self.grid[point.0 + point.1*self.width] = val;
		}
	}

	pub fn get(&self, point: Point) -> bool {
		point.0 < self.width && point.1 < self.height && self.grid[point.0 + point.1*self.width]
	}

	pub fn len(&self) -> usize {
		self.width * self.height
	}
}


#[derive(PartialEq, Copy, Clone)]
pub enum Dir {
	Right,
	Left,
	Up,
	Down
}

pub const DIRECTIONS: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Down, Dir::Up];

impl Dir {
	pub fn opposite(self) -> Dir {
		DIRECTIONS[self as usize]
	}

	pub fn tuple(&self) -> (usize, usize) {
		match self {
			Dir::Right => (2, 1),
			Dir::Left => (0, 1),
			Dir::Up => (1, 2),
			Dir::Down => (1, 0)
		}
	}
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point(pub usize, pub usize);

impl std::ops::Add<Dir> for Point {
	type Output = Point;

	fn add(self, rhs: Dir) -> Point {
		let tuple = rhs.tuple();

		Point(self.0 + tuple.0-1, self.1 + tuple.1-1)
	}
}
