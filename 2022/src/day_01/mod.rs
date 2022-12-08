use std::fs;

use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = Vec<u32>;

pub fn solve() -> (Output, Output) {
	let raw = fs::read_to_string("./src/day_01/input.txt").unwrap();
	let input = input::read(raw);
	(part1::solve(&input), part2::solve(&input))
}
