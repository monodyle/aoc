use crate::utils::{LinesToUInt, Solution};
use std::fs::read_to_string;

fn parse_input(input: String) -> Vec<usize> {
    input.to_vec_uint()
}

pub fn solve() -> (Solution, Solution) {
    let input = parse_input(read_to_string("input/day01").unwrap());
    (
        Solution::UInt(part_one(&input)),
        Solution::UInt(part_two(&input)),
    )
}

fn part_one(input: &[usize]) -> usize {
    input
        .iter()
        .enumerate()
        .map(|(index, _)| match index {
            0 => 0,
            x => {
                if input[x] > input[x - 1] {
                    1
                } else {
                    0
                }
            }
        })
        .sum()
}

#[test]
fn test_part_one() {
    let input = parse_input(
        "199
        200
        208
        210
        200
        207
        240
        269
        260
        263"
        .to_string(),
    );
    assert_eq!(part_one(&input), 7);
}

fn part_two(input: &[usize]) -> usize {
    let sum: Vec<usize> = (0..(input.len() - 2))
        .map(|i| input[i] + input[i + 1] + input[i + 2])
        .collect();
    sum.iter()
        .enumerate()
        .map(|(index, _)| match index {
            0 => 0,
            x => {
                if sum[x] > sum[x - 1] {
                    1
                } else {
                    0
                }
            }
        })
        .sum()
}

#[test]
fn test_part_two() {
    let input = parse_input(
        "199
        200
        208
        210
        200
        207
        240
        269
        260
        263"
        .to_string(),
    );
    assert_eq!(part_two(&input), 5);
}
