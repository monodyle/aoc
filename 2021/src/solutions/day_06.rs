use crate::utils::{load_input, Solution};

fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

pub fn solve() -> (Solution, Solution) {
    let input = parse_input(load_input("06"));

    (
        Solution::UInt(counter(&input, 80)),
        Solution::UInt(counter(&input, 256)),
    )
}

fn counter(input: &Vec<usize>, days: usize) -> usize {
    let mut timers = input.iter().fold([0; 9], |mut acc, &v| {
        acc[v] += 1;
        acc
    });

    for _ in 1..=days {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }

    timers.iter().sum()
}

#[test]
fn test_part_one() {
    let input = "3,4,3,1,2".to_string();
    assert_eq!(counter(&parse_input(input), 80), 5934);
}

#[test]
fn test_part_two() {
    let input = "3,4,3,1,2".to_string();
    assert_eq!(counter(&parse_input(input), 256), 26984457539);
}
