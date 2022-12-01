use std::fs;

use super::Input;

pub fn read() -> Input {
	let file = fs::read_to_string("./src/day_01/input.txt").unwrap();
	file.trim().split("\n\n").map(parse_calories).collect()
}

fn parse_calories(value: &str) -> u32 {
	value.lines().flat_map(|l| l.parse::<u32>()).sum::<u32>()
}
