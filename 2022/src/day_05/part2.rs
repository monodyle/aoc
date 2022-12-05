use aoc2022lib::Output;

use super::{get_to_of_stacks, Input, Procedure};

pub fn solve(input: &Input) -> Output {
    let (stacks, procedure) = <&Input>::clone(&input);
    let mut stacks = stacks.clone();

    for Procedure { mv, fr, to } in procedure {
		let mut moved = "".to_string();
        for _ in 0..*mv {
            let c = stacks.get_mut(fr).unwrap().remove(0);
			moved += &c.to_string();
        }
		stacks.get_mut(to).unwrap().insert_str(0, &moved);
    }

    Output::Str(get_to_of_stacks(stacks))
}
