use std::{fs::read_to_string, vec};

use crate::utils::Solution;

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<Vec<usize>>,
    checker: Vec<Vec<bool>>,
}

impl Board {
    fn new(numbers: Vec<Vec<usize>>) -> Self {
        let checker = vec![vec![false; 5]; 5];
        Self { numbers, checker }
    }

    fn call_mark(&mut self, call: usize) {
        self.numbers
            .iter()
            .enumerate()
            .for_each(|(row_index, row)| {
                row.iter().enumerate().for_each(|(number_index, number)| {
                    if *number == call {
                        self.checker[row_index][number_index] = true;
                    }
                })
            });
    }

    fn is_bingo(&self) -> bool {
        for i in 0..5 {
            if self.checker[i].iter().all(|v| *v) || self.checker.iter().map(|v| v[i]).all(|v| v) {
                return true;
            }
        }
        false
    }

    fn rest_sum(&self) -> usize {
        let mut sum = 0;
        for (i, row) in self.numbers.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if !self.checker[i][j] {
                    sum += item;
                }
            }
        }
        sum
    }
}

fn parse_input(input: String) -> (Vec<usize>, Vec<Board>) {
    let mut splitter: Vec<String> = input.split("\n\n").map(|v| v.to_string()).collect();
    let caller = splitter
        .drain(0..1)
        .collect::<String>()
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut boards = vec![];
    for board in splitter {
        let numbers = board
            .split("\n")
            .map(|v| {
                v.trim()
                    .split_whitespace()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        boards.push(Board::new(numbers));
    }
    (caller, boards)
}

pub fn solve() -> (Solution, Solution) {
    let (caller, mut boards) = parse_input(read_to_string("input/day04").unwrap());

    (
        Solution::UInt(part_one(&caller, &mut boards)),
        Solution::UInt(part_two(&caller, &mut boards)),
    )
}

fn part_one(caller: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    let mut bingo_call = 0;
    let mut bingo_index = 0;

    'find_bingo: for (call_index, call_number) in caller.iter().enumerate() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            board.call_mark(*call_number);
            if call_index >= 5 && board.is_bingo() {
                bingo_call = *call_number;
                bingo_index = board_index;
                break 'find_bingo;
            }
        }
    }

    let score = boards[bingo_index].rest_sum();
    score * bingo_call
}

#[test]
fn test_part_one() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
        .to_string();
    let (caller, mut boards) = parse_input(input);
    assert_eq!(part_one(&caller, &mut boards), 4512);
}

fn part_two(caller: &Vec<usize>, boards: &mut Vec<Board>) -> usize {
    let mut bingo_boards: Vec<usize> = vec![0];

    let mut last_bingo_number = 0;
    let mut last_bingo_board: Board = Board::new(vec![vec![0; 5]; 5]);

    for (call_index, call_number) in caller.iter().enumerate() {
        for (board_index, board) in boards.iter_mut().enumerate() {
            board.call_mark(*call_number);
            if call_index >= 5 {
                if bingo_boards.contains(&board_index) {
                    continue;
                }
                if board.is_bingo() {
                    last_bingo_number = caller[call_index];
                    last_bingo_board = board.clone();
                    bingo_boards.push(board_index)
                }
            }
        }
    }

    let score = last_bingo_board.rest_sum();
    println!("{}", score);
    println!("{}", last_bingo_number);
    score * last_bingo_number
}

#[test]
fn test_part_two() {
    let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
        .to_string();
    let (caller, mut boards) = parse_input(input);
    assert_eq!(part_two(&caller, &mut boards), 1924);
}
