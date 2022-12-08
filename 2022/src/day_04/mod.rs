use std::fs;

use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Range = (u32, u32);
pub type Pair = (Range, Range);

pub type Input = Vec<Pair>;

pub fn solve() -> (Output, Output) {
	let raw = fs::read_to_string("./src/day_04/example.txt").unwrap();
	let input = input::read(raw);
	(part1::solve(&input), part2::solve(&input))
}
