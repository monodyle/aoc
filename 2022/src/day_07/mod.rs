use std::collections::HashMap;

use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = HashMap<String, u32>;

pub fn solve() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}
