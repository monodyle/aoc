use aoc2022lib::Output;

use super::{Input, Move};

pub fn solve(input: &Input) -> Output {
    let mut score = 0;
    for (a, b) in input {
        let their = &Move::from_str(a);
        let me = &(match b.as_str() {
            "X" => their.derive(false),
            "Y" => *their,
            "Z" => their.derive(true),
            _ => unreachable!(),
        });
        score += me.to_score() + me.fight(their).to_score();
    }
    Output::U32(score)
}
