use crate::utils::{load_input, Solution};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point(usize, usize);

struct Segment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Segment {
    fn from_string(s: String) -> Self {
        let mut splitter = s.split(" -> ");

        let mut from = splitter.next().unwrap().trim().split(",");
        let x1 = from.next().unwrap().trim().parse::<usize>().unwrap();
        let y1 = from.next().unwrap().trim().parse::<usize>().unwrap();

        let mut to = splitter.next().unwrap().trim().split(",");
        let x2 = to.next().unwrap().trim().parse::<usize>().unwrap();
        let y2 = to.next().unwrap().trim().parse::<usize>().unwrap();

        Self { x1, y1, x2, y2 }
    }

    fn is_straight(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if self.is_straight() {
            if self.x1 == self.x2 {
                if self.y1 < self.y2 {
                    (self.y1..=self.y2).for_each(|y| points.push(Point(self.x1, y)));
                } else {
                    (self.y2..=self.y1)
                        .rev()
                        .for_each(|y| points.push(Point(self.x1, y)));
                }
            } else {
                if self.x1 < self.x2 {
                    (self.x1..=self.x2).for_each(|x| points.push(Point(x, self.y1)));
                } else {
                    (self.x2..=self.x1)
                        .rev()
                        .for_each(|x| points.push(Point(x, self.y1)));
                }
            }
        } else {
            let xrange = if self.x1 < self.x2 {
                (self.x1..=self.x2).collect::<Vec<usize>>()
            } else {
                (self.x2..=self.x1).rev().collect::<Vec<usize>>()
            };

            let yrange = if self.y1 < self.y2 {
                (self.y1..=self.y2).collect::<Vec<usize>>()
            } else {
                (self.y2..=self.y1).rev().collect::<Vec<usize>>()
            };

            xrange
                .iter()
                .zip(yrange)
                .for_each(|(&x, y)| points.push(Point(x, y)));
        }

        points
    }
}

fn parse_input(input: String) -> Vec<Segment> {
    input
        .trim()
        .lines()
        .map(|l| Segment::from_string(l.to_string()))
        .collect()
}

pub fn solve() -> (Solution, Solution) {
    let input = parse_input(load_input("05"));
    (
        Solution::UInt(part_one(&input)),
        Solution::UInt(part_two(&input)),
    )
}

fn part_one(input: &Vec<Segment>) -> usize {
    let mut map = HashMap::<Point, usize>::new();

    input.iter().filter(|s| s.is_straight()).for_each(|s| {
        s.points().iter().for_each(|p| {
            *map.entry(*p).or_insert(0) += 1;
        });
    });

    map.iter().filter(|(_, v)| *v > &1).count()
}

#[test]
fn test_part_one() {
    let input = parse_input(
        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2"
            .to_string(),
    );
    assert_eq!(part_one(&input), 5);
}

fn part_two(input: &Vec<Segment>) -> usize {
    let mut map = HashMap::<Point, usize>::new();

    input.iter().for_each(|s| {
        s.points().iter().for_each(|p| {
            *map.entry(*p).or_insert(0) += 1;
        });
    });

    map.iter().filter(|(_, v)| *v > &1).count()
}

#[test]
fn test_part_two() {
    let input = parse_input(
        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2"
            .to_string(),
    );
    assert_eq!(part_two(&input), 12);
}
