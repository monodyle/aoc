use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
    Output::U32(
        input
            .iter()
            .filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1))
            .count()
            .try_into()
            .unwrap(),
    )
}

/* pub fn solve(input: &Input) -> Output {
    let mut result = 0;

    for (first, second) in input {
        if (first.0 <= second.0 && first.1 >= second.1)
            || (second.0 <= first.0 && second.1 >= first.1)
        {
            result += 1
        }
    }

    Output::U32(result)
} */
