/*
	Ici 'path_from_connected' donne le chemin de type 'Path' orienté de point1
	vers point2, 'path_to_center' est une fonction auxiliaire donnant le
	chemin vers le point 'center' de ways
*/

use crate::utils::Point;
use crate::navigation::{Ways, Path};

pub fn path_from_connected(ways: &Ways, point1: Point, point2: Point) -> Path {
	let mut path1 = path_to_center(ways, point1);
	let mut path2 = path_to_center(ways, point2);
	let l1 = path1.moves.len();
	let l2 = path2.moves.len();

	let mut index = 1;

	while l1 >= index && l2 >= index && path1.moves[l1-index] == path2.moves[l2-index] {
		index += 1;
	}

	path1.moves.truncate(l1+1-index);
	path2.moves.truncate(l2+1-index);
	path2.moves.reverse();

	path1.moves.append(&mut path2.moves.into_iter().map(|dir| dir.opposite()).collect());

	path1
}

pub fn path_to_center(ways: &Ways, start: Point) -> Path {
	let mut moves = Vec::new();
	let mut curPoint = start;

	let center = ways.center();
	while curPoint != center {
		let dir = ways.get_dir(curPoint).unwrap();
		moves.push(dir);
		curPoint = curPoint + dir;
	}

	Path {
		start,
		moves
	}
}
