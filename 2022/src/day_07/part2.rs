use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
    let outermost = input[""];
    let mut dirs = input.values().collect::<Vec<_>>();
    dirs.sort();
    Output::U32(
        *dirs.into_iter()
            .find(|&&s| 70000000 - (outermost - s) >= 30000000)
            .unwrap(),
    )
}
