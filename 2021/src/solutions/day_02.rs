use std::fs::read_to_string;

use crate::utils::Solution;

struct Position {
    horizon: isize,
    depth: isize,
    aim: isize,
}

enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

fn parse_input(input: String) -> Vec<Direction> {
    input
        .trim_end()
        .lines()
        .map(|line| line.trim().split(" "))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .map(|pair| match pair {
            ("forward", n) => Direction::Forward(n),
            ("down", n) => Direction::Down(n),
            ("up", n) => Direction::Up(n),
            _ => unreachable!(),
        })
        .collect()
}

pub fn solve() -> (Solution, Solution) {
    let input: Vec<Direction> = parse_input(read_to_string("input/day02").unwrap());

    (
        Solution::Int(part_one(&input)),
        Solution::Int(part_two(&input)),
    )
}

fn part_one(input: &Vec<Direction>) -> isize {
    let result = input.iter().fold(
        Position {
            horizon: 0,
            depth: 0,
            aim: 0,
        },
        |acc, instruction| match instruction {
            Direction::Forward(n) => Position {
                horizon: acc.horizon + n,
                ..acc
            },
            Direction::Up(n) => Position {
                depth: acc.depth - n,
                ..acc
            },
            Direction::Down(n) => Position {
                depth: acc.depth + n,
                ..acc
            },
        },
    );

    result.horizon * result.depth
}

#[test]
fn test_part_one() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "
    .to_string();
    assert_eq!(part_one(&parse_input(input)), 150);
}

fn part_two(input: &Vec<Direction>) -> isize {
    let result = input.iter().fold(
        Position {
            horizon: 0,
            depth: 0,
            aim: 0,
        },
        |acc, instruction| match instruction {
            Direction::Forward(n) => Position {
                horizon: acc.horizon + n,
                depth: acc.depth + (acc.aim * n),
                ..acc
            },
            Direction::Up(n) => Position {
                aim: acc.aim - n,
                ..acc
            },
            Direction::Down(n) => Position {
                aim: acc.aim + n,
                ..acc
            },
        },
    );

    result.horizon * result.depth
}

#[test]
fn test_part_two() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    "
    .to_string();
    assert_eq!(part_two(&parse_input(input)), 900);
}
