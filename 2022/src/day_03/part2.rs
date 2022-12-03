use aoc2022lib::Output;

use super::{Input, char_to_priority};

pub fn solve(input: &Input) -> Output {
    let mut sum = 0;

    for i in 0..(input.len() / 3) {
		let index = i * 3;
        let first = input[index].chars().collect::<Vec<_>>();
        let second = input[index+1].chars().collect::<Vec<_>>();
        let third = input[index+2].chars().collect::<Vec<_>>();
		for c in first {
            if second.contains(&c) && third.contains(&c) {
                sum += char_to_priority(c);
                break;
            }
        }
	}

    Output::U32(sum)
}
