use crate::utils::{load_input, Solution};

fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

pub fn solve() -> (Solution, Solution) {
    let input = parse_input(load_input("07"));
    (
        Solution::UInt(part_one(&input)),
        Solution::UInt(part_two(&input)),
    )
}

fn part_one(input: &[usize]) -> usize {
    let max = *input.iter().max().unwrap();
    (0..=max)
        .into_iter()
        .map(|dist| {
            input
                .into_iter()
                .map(|&pos| (pos as isize - dist as isize).abs() as usize)
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part_one() {
    let input = parse_input("16,1,2,0,4,2,7,1,2,14".to_string());
    assert_eq!(part_one(&input), 37);
}

/**
 * Triangular number:
 * a(n) = binomial(n+1,2) = n*(n+1)/2 = 0 + 1 + 2 + ... + n.
 * https://en.wikipedia.org/wiki/Triangular_number#Formula
 */

fn part_two(input: &[usize]) -> usize {
    let max = *input.iter().max().unwrap();
    (0..=max)
        .into_iter()
        .map(|dist| {
            input
                .into_iter()
                .map(|&pos| {
                    let steps = (pos as isize - dist as isize).abs() as usize;
                    // (1..=steps).into_iter().sum::<usize>()
                    steps * (steps + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part_two() {
    let input = parse_input("16,1,2,0,4,2,7,1,2,14".to_string());
    assert_eq!(part_two(&input), 168);
}
