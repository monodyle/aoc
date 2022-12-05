use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .filter(|(a, b)| !(a.1 < b.0 || b.1 < a.0))
            .count()
            .try_into()
            .unwrap(),
    )
}

/* pub fn solve(input: &Input) -> Output {
    let mut non_overlap = 0;

    for (first, second) in input {
        if first.1 < second.0 || second.1 < first.0 {
            non_overlap += 1;
        }
    }

    Output::U32((input.len() - non_overlap).try_into().unwrap())
} */
