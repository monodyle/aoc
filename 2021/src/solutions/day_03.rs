use crate::utils::{Solution, load_input};

fn parse_input(input: String) -> Vec<Vec<String>> {
    input
        .trim_end()
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_string()).collect())
        .collect()
}

fn bin_str_to_dec(s: String) -> isize {
    isize::from_str_radix(&s, 2).unwrap()
}

pub fn solve() -> (Solution, Solution) {
    let input = parse_input(load_input("03"));

    (
        Solution::Int(part_one(&input)),
        Solution::Int(part_two(&input)),
    )
}

fn part_one(input: &Vec<Vec<String>>) -> isize {
    let l = input.len();
    let mut ones: Vec<usize> = vec![0; input[0].len()];
    for item in input.iter() {
        for (i, v) in item.iter().enumerate() {
            if v == "1" {
                ones[i] += 1;
            }
        }
    }

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for i in ones {
        if i < l / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    bin_str_to_dec(gamma) * bin_str_to_dec(epsilon)
}

#[test]
fn test_part_one() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
        .to_string();
    assert_eq!(part_one(&parse_input(input)), 198);
}

fn part_two(input: &Vec<Vec<String>>) -> isize {
    let l = input.len();
    let mut oxy_remains = (0..l).collect::<Vec<usize>>();
    let mut co2_remains = (0..l).collect::<Vec<usize>>();
    let mut index = 0;

    while index < input[0].len() {
        let mut oxy_ones: Vec<usize> = Vec::new();
        let mut oxy_zeroes: Vec<usize> = Vec::new();
        let mut co2_ones: Vec<usize> = Vec::new();
        let mut co2_zeroes: Vec<usize> = Vec::new();

        if oxy_remains.len() > 1 {
            for v in oxy_remains.iter() {
                if input[*v][index] == "1" {
                    oxy_ones.push(*v)
                } else {
                    oxy_zeroes.push(*v)
                }
            }
            if oxy_ones.len() >= oxy_zeroes.len() {
                oxy_remains = oxy_ones.to_owned();
            } else {
                oxy_remains = oxy_zeroes.to_owned();
            }
        }

        if co2_remains.len() > 1 {
            for v in co2_remains.iter() {
                if input[*v][index] == "1" {
                    co2_ones.push(*v)
                } else {
                    co2_zeroes.push(*v)
                }
            }
            if co2_ones.len() < co2_zeroes.len() {
                co2_remains = co2_ones.to_owned();
            } else {
                co2_remains = co2_zeroes.to_owned();
            }
        }

        index += 1;
    }

    let oxy = bin_str_to_dec(input[oxy_remains[0]].join(""));
    let co2 = bin_str_to_dec(input[co2_remains[0]].join(""));

    oxy * co2
}

#[test]
fn test_part_two() {
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"
        .to_string();
    assert_eq!(part_two(&parse_input(input)), 230);
}
