use std::fs;

use super::{Input, Pair, Range};

pub fn read() -> Input {
	let file = fs::read_to_string("./src/day_04/input.txt").unwrap();
	file.trim().lines().map(parse_pair).collect()
}

fn parse_pair(value: &str) -> Pair {
	let (first, second) = value.split_once(',').unwrap();
	(parse_range(first), parse_range(second))
}

fn parse_range(value: &str) -> Range {
	let (from, to) = value.split_once('-').unwrap();
	(from.parse::<u32>().unwrap(), to.parse::<u32>().unwrap())
}

/* #[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parser() {
		let input = read();
		assert_eq!(
			input,
			vec![
				((2,4),(6,8)),
				((2,3),(4,5)),
				((5,7),(7,9)),
				((2,8),(3,7)),
				((6,6),(4,6)),
				((2,6),(4,8)),
			]
		);
	}
} */
