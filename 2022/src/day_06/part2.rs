use aoc2022lib::Output;

use super::{Input, get_index_of_maker};

pub fn solve(input: &Input) -> Output {
    Output::U32(get_index_of_maker(input, 14))
}
