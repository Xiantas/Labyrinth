/*
	'from_point_generate' modifie un labyrinthe vierge en place pour obtenir
	un labyrinthe parfait en enlevant des murs entre un point colorié et
	un autre non colorié qui est ensuite colorié jusqu'à ce qu'il n'y aie
	plus de murs à enlever
*/

#![allow(non_snake_case)]

use crate::labyrinth::Labyrinth;
use crate::utils::{Coloring, Point, DIRECTIONS};

use rand::Rng;

pub fn from_point_generate(labyrinth: &mut Labyrinth, starting_opt: Option<Point>) {
	let width = labyrinth.width();
	let height = labyrinth.height();

	let start_point = match starting_opt.filter(|point| labyrinth.is_valid_point(*point)) {
		None =>
			Point(rand::thread_rng().gen_range(0, width),
			 rand::thread_rng().gen_range(0, height)),
		Some(point) =>
			point
	};

	let mut seen = Coloring::new(&labyrinth);

	seen.set(start_point, true);
	let mut potential_ways = Vec::with_capacity(labyrinth.width()*labyrinth.height());

	for dir in &DIRECTIONS {
		if !labyrinth.goes_out(start_point, *dir) {
			potential_ways.push((start_point, *dir));
		}
	}

	while !potential_ways.is_empty() {
		let randIndex: usize = rand::thread_rng().gen_range(0, potential_ways.len());

		let (point, dir) = potential_ways.remove(randIndex);

		let newPoint = point + dir;

		if !seen.get(newPoint) {
			labyrinth.set_way(dir, point, true);
			seen.set(newPoint, true);
			for d in &DIRECTIONS {
				if !labyrinth.goes_out(newPoint, *d) && !seen.get(newPoint+*d) {
					potential_ways.push((newPoint, *d));
				}
			}
		}
	}
}
