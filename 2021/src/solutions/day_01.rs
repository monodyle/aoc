use crate::utils::LinesToVec32;
use std::fs::read_to_string;

pub fn solve() -> (u32, u32) {
    let input = read_to_string("input/day01").unwrap().to_vec32();
    (part_one(&input), part_two(&input))
}

fn part_one(input: &[u32]) -> u32 {
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
    let input = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
    .to_string()
    .to_vec32();
    assert_eq!(part_one(&input), 7);
}

fn part_two(input: &[u32]) -> u32 {
    let sum: Vec<u32> = (0..(input.len() - 2))
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
    let input = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
    .to_string()
    .to_vec32();
    assert_eq!(part_two(&input), 5);
}
