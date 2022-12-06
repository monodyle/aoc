use std::collections::HashSet;

use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = Vec<char>;

pub fn get_index_of_maker(input: &Input, size: u32) -> u32 {
	let windows = input.windows(size.try_into().unwrap());
	let mut i = 0;
	for window in windows {
		if window.iter().collect::<HashSet<_>>().len() == window.len() {
			return i + window.len() as u32
		}
		i += 1;
    }
	0
}

pub fn solve() -> (Output, Output) {
	let input = input::read();
	(part1::solve(&input), part2::solve(&input))
}
