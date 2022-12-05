use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = Vec<String>;

pub fn char_to_priority(c: char) -> u32 {
	let p = c as u32;
	match p {
		97..=122 => p - 96,
		65..=90 => p - 38,
		_ => unreachable!(),
	}
}

pub fn solve() -> (Output, Output) {
	let input = input::read();
	(part1::solve(&input), part2::solve(&input))
}
