use aoc2022lib::Output;

use super::{Input, char_to_priority};

pub fn solve(input: &Input) -> Output {
    let mut sum = 0;
    for line in input.iter() {
		let middle = line.len() / 2;
        let first = line[..middle].chars().collect::<Vec<_>>();
        let second = line[middle..].chars().collect::<Vec<_>>();
        for c in first {
            if second.contains(&c) {
                sum += char_to_priority(c);
                break;
            }
        }
    }
    Output::U32(sum)
}
