use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = Vec<String>;

pub fn char_to_priority(c: char) -> u32 {
    let mut priority = c as u32;
    priority -= 38;
    if priority >= 59 {
        priority -= 58;
    }
	priority
}

pub fn solve() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}
