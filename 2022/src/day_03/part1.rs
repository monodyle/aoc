use aoc2022lib::Output;

use super::{char_to_priority, Input};

pub fn solve(input: &Input) -> Output {
    let mut sum = 0;
    for line in input.iter() {
        let middle = line.len() / 2;
        let first = line[..middle].chars().collect::<Vec<_>>();
        let second = &line[middle..];
        for c in first {
            if second.contains(&c.to_string()) {
                sum += char_to_priority(c);
                break;
            }
        }
    }
    Output::U32(sum)
}
