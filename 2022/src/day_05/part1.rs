use aoc2022lib::Output;

use super::{Input, Procedure, get_to_of_stacks};

pub fn solve(input: &Input) -> Output {
    let (stacks, procedure) = <&Input>::clone(&input);
    let mut stacks = stacks.clone();

    for Procedure { mv, fr, to } in procedure {
        for _ in 0..*mv {
            let c = stacks.get_mut(fr).unwrap().remove(0);
            stacks.get_mut(to).unwrap().insert(0, c);
        }
    }

    Output::Str(get_to_of_stacks(stacks))
}
