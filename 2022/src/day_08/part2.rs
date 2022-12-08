use aoc2022lib::Output;

use super::Input;

pub fn looking(h: u32, view: Vec<u32>) -> u32 {
    let mut trees = 0;
    for tree in view {
        trees += 1;
        if tree >= h {
            break;
        }
    }
    trees
}

pub fn solve(input: &Input) -> Output {
    let mut max = 0;

    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            let h = input[y][x];
            // look up
            let up = looking(h, input[..y].iter().map(|r| r[x]).rev().collect::<Vec<_>>());
            let left = looking(h, input[y][..x].iter().rev().copied().collect::<Vec<_>>());
            let down = looking(h, input[y + 1..].iter().map(|r| r[x]).collect::<Vec<_>>());
            let right = looking(h, input[y][x + 1..].to_vec());
            max = max.max(up * left * down * right)
        }
    }

    Output::U32(max)
}
