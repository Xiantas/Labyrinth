/*
	Ici est défini 'Labyrinth' ainsi que les différentes méthodes d'interaction
	avec cette structure de données
*/
#![allow(non_snake_case)]

use crate::utils::{Dir, Point, DIRECTIONS};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::utils::Coloring;

pub struct Labyrinth {
	width: usize,
	height: usize,
	links: Vec<bool>
}

impl Labyrinth {
	pub fn new(width: usize, height: usize) -> Labyrinth {
		Labyrinth {
			width,
			height,
			links: vec![false; height * (2*width-1) - height],
		}
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn is_valid_point(&self, point: Point) -> bool {
		point.0 < self.width && point.1 < self.height
	}


	pub fn has_dir(&self, dir: Dir, point: Point) -> bool {
		if !self.is_valid_point(point) {
			panic!("Invalid point: ({}, {}) for lab: ({}, {})", point.0, point.1, self.width, self.height);
		}

		match dir {
			Dir::Right => point.0 != self.width-1 && self.links[point.0 + point.1 * (self.width-1)],
			Dir::Left => point.0 != 0 && self.links[point.0-1 + point.1 * (self.width-1)],
			Dir::Up => point.1 != self.height-1 && self.links[self.height * (self.width-1) + point.0 + point.1 * self.width],
			Dir::Down => point.1 != 0 && self.links[self.height * (self.width-1) + point.0 + (point.1-1) * self.width]
		}
	}


	pub fn set_way(&mut self, dir: Dir, point: Point, val: bool) {
		if !self.is_valid_point(point) {
			panic!("Invalid point: ({}, {}) for lab: ({}, {})", point.0, point.1, self.width, self.height);
		}

		match dir {
			Dir::Right =>
				self.links[point.0 + point.1 * (self.width-1)] = val,
			Dir::Left if point.0 != 0 =>
				self.links[point.0-1 + point.1 * (self.width-1)] = val,
			Dir::Up if point.1 != self.height-1 =>
				self.links[self.height * (self.width-1) + point.0 + point.1 * self.width] = val,
			Dir::Down if point.1 != 0 =>
				self.links[self.height * (self.width-1) + point.0 + (point.1-1) * self.width] = val,
			_ => ()
		}
	}


	pub fn is_on_edges(&self, point: Point) -> bool {
		point.0 == 0 || point.1 == 0 || point.0 == self.width-1 || point.1 == self.height-1
	}

	pub fn goes_out(&self, point: Point, dir: Dir) -> bool {
		(point.0 == 0 && dir == Dir::Left) ||
		(point.1 == 0 && dir == Dir::Down) ||
		(point.0 == self.width-1 && dir == Dir::Right) ||
		(point.1 == self.height-1 && dir == Dir::Up)
	}

	pub fn is_fully_connected(&self) -> bool {
		let mut queue: Vec<Point> = Vec::with_capacity(self.width + self.height);
		queue.push(Point(0,0));

		let mut nb_seen = 0;
		let mut seen = Coloring::new(&self);
		seen.set(Point(0,0), true);

		while let Some(point) = queue.pop() {
			nb_seen += 1;
			for dir in &DIRECTIONS {
				if self.has_dir(*dir, point) && !seen.get(point + *dir) {
					seen.set(point + *dir, true);
					queue.push(point+*dir);
				}
			}

			if queue.len() > self.width * self.height {
				println!("the queue was to small !")
			}
		}

		nb_seen == seen.len()
	}

	pub fn render(&self, canvas: &mut Canvas<Window>) {
		canvas.set_draw_color((255, 255, 255));
		
		let _ = canvas.draw_line((10, 990), (10, 990-20*self.height as i32));
		let _ = canvas.draw_line((10, 990), (10+20*self.width as i32, 990));
		let _ = canvas.draw_line((10, 990-20*self.height as i32), (10+20*self.width as i32, 990-20*self.height as i32));
		let _ = canvas.draw_line((10+20*self.width as i32, 990), (10+20*self.width as i32, 990-20*self.height as i32));

		for y in 0..self.height-1 {
			for x in 0..self.width {
				if !self.has_dir(Dir::Up, Point(x, y)) {
					let _ = canvas.draw_line((10+20*x as i32, 970-20*y as i32), (30+20*x as i32, 970-20*y as i32));
				}
			}
		}

		for y in 0..self.height {
			for x in 0..self.width-1 {
				if !self.has_dir(Dir::Right, Point(x, y)) {
					let _ = canvas.draw_line((30+20*x as i32, 970-20*y as i32), (30+20*x as i32, 990-20*y as i32));
				}
			}
		}
	}

	pub fn debug(&self) {
		println!("Debug laby:\n  > w: {}\n  > h: {}\n  > l: {:?}", self.width, self.height, self.links);
	}
}
