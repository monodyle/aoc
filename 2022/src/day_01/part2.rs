use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
	let mut input = input.clone();
	input.sort_by(|a, b| b.cmp(a));
	Output::U32(input.iter().take(3).sum::<u32>())
}
