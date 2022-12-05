use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
    Output::U32(input.iter().copied().max().unwrap())
}
