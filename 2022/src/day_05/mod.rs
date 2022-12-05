use std::collections::BTreeMap;

use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Stacks = BTreeMap<u8, String>;

#[derive(Debug, PartialEq, Eq)]
pub struct Procedure {
	pub mv: u8,
	pub fr: u8,
	pub to: u8,
}

pub type Input = (Stacks, Vec<Procedure>);

impl Procedure {
	pub fn new(mv: u8, fr: u8, to: u8) -> Self {
		Self { mv, fr, to }
	}
}

pub fn get_to_of_stacks(stacks: Stacks) -> String {
	stacks
		.values()
		.map(|i| i.chars().next().unwrap().to_string())
		.collect::<Vec<_>>()
		.join("")
}

pub fn solve() -> (Output, Output) {
	let input = input::read();
	(part1::solve(&input), part2::solve(&input))
}
