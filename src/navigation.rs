/*
	Ici sont définis deux structures relatives au déplacement dans un labyrinthe:
		- 'Ways' est associé à un labyrinthe et donne sur chaque case la direction
		  à prendre pour aller vers center, cette structure permet de trouver le
		  chemin entre de points plus rapidement
		- 'Path' décrit un chemin par un point de départ et une suite de directions prises
*/

#![allow(non_snake_case)]

use crate::utils::{Point, Dir, Coloring, DIRECTIONS};
use crate::labyrinth::Labyrinth;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ways {
	width: usize,
	height: usize,
	dirs: Vec<Dir>,
	center: Point
}

impl Ways {
	pub fn generate(labyrinth: &Labyrinth, center: Option<Point>) -> Ways {
		let center = match center.filter(|point| labyrinth.is_valid_point(*point)) {
			None =>
				Point(labyrinth.width()/2, labyrinth.height()/2),
			Some(point) =>
				point
		};

		let mut ways = Ways::create(labyrinth.width(), labyrinth.height(), center);

		let mut pointsToSee = Vec::with_capacity(ways.width + ways.height);
		let mut seen = Coloring::new(labyrinth);

		pointsToSee.push(center);
		seen.set(center, true);

		while let Some(point) = pointsToSee.pop() {
			for dir in &DIRECTIONS {
				if labyrinth.has_dir(*dir, point) && !seen.get(point + *dir) {
					let newPoint = point + *dir;
					seen.set(newPoint, true);
					pointsToSee.push(newPoint);
					ways.set_dir(newPoint, dir.opposite());
				}
			}
		}

		ways
	}


	pub fn width(&self) -> usize {self.width}
	pub fn height(&self) -> usize {self.height}
	pub fn center(&self) -> Point {self.center}

	pub fn get_dir(&self, point: Point) -> Option<Dir> {
		if point == self.center {
			None
		} else {
			Some(self.dirs[point.0 + point.1*self.width])
		}
	}

	fn set_dir(&mut self, point: Point, dir: Dir) {
		if point.0 < self.width && point.1 < self.height {
			self.dirs[point.0 + point.1*self.width] = dir;
		} else {
			println!("Invalid point: ({}, {}) for a Ways: ({}, {})", point.0, point.1, self.width, self.height);
		}
	}

	fn create(width: usize, height: usize, center: Point) -> Ways {
		let dirs = vec![Dir::Right; width*height];

		Ways {
			width,
			height,
			dirs,
			center
		}
	}
}

pub struct Path {
	pub start: Point,
	pub moves: Vec<Dir>
}

impl Path {
	pub fn ending_point(&self) -> Option<Point> {
		let mut res = self.start;

		for dir in &self.moves {
			if (res.0 == 0 && *dir == Dir::Right) || (res.1 == 0 && *dir == Dir::Down) {
				return None
			} else {
				res = res + *dir;
			}
		}

		Some(res)
	}

	pub fn render(&self, canvas: &mut Canvas<Window>) {
		use sdl2::rect::Rect;

		canvas.set_draw_color((0,0,255));
		let _ = canvas.fill_rect(Some(Rect::new(15+20*self.start.0 as i32, 975-20*self.start.1 as i32, 11, 11)));

		let mut curPoint = self.start;
		let offsets = [(-4i32, -15i32, 20u32, 11u32), (5, -15, 20, 11), (5, -15, 11, 20), (5, -25, 11, 20)];

		for dir in &self.moves {
			curPoint = curPoint + *dir;

			let _ = canvas.fill_rect(Some(Rect::new(
				10+20*curPoint.0 as i32 + offsets[*dir as usize].0,
				990-20*curPoint.1 as i32 + offsets[*dir as usize].1,
				offsets[*dir as usize].2,
				offsets[*dir as usize].3)));
		}
	}
}
