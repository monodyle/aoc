use std::{fs::read_to_string, vec};

use crate::utils::Solution;

type Board<T> = Vec<Vec<T>>;

fn parse_input(input: String) -> (Vec<usize>, Vec<Board<usize>>) {
    let mut splitter: Vec<String> = input.split("\n\n").map(|v| v.to_string()).collect();
    let caller = splitter
        .drain(0..1)
        .collect::<String>()
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let boards: Vec<Board<usize>> = splitter
        .iter()
        .map(|v| {
            v.trim()
                .split("\n")
                .map(|n| {
                    n.trim()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    (caller, boards)
}

pub fn solve() -> (Solution, Solution) {
    let (caller, boards) = parse_input(read_to_string("input/day04").unwrap());

    (
        Solution::UInt(part_one(&caller, &boards)),
        Solution::UInt(part_two(&caller, &boards)),
    )
}

fn is_bingo(board: &Board<bool>) -> bool {
    for i in 0..5 {
        if board[i].iter().all(|v| *v) || board.iter().map(|v| v[i]).all(|v| v) {
            return true;
        }
    }
    false
}

fn part_one(caller: &Vec<usize>, boards: &Vec<Board<usize>>) -> usize {
    let mut check_boards: Vec<Board<bool>> = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut bingo_number = 0;
    let mut bingo_board = 0;

    'find_bingo: for (call_index, call_number) in caller.iter().enumerate() {
        for (board_index, board) in boards.iter().enumerate() {
            for (row_index, row) in board.iter().enumerate() {
                for (item_index, item) in row.iter().enumerate() {
                    if item == call_number {
                        check_boards[board_index][row_index][item_index] = true;
                    }
                }
            }
        }
        if call_index >= 5 {
            for (board_index, board) in check_boards.iter().enumerate() {
                if is_bingo(&board) {
                    bingo_number = caller[call_index];
                    bingo_board = board_index;
                    break 'find_bingo;
                }
            }
        }
    }

    let mut score = 0;
    for (row_index, row) in boards[bingo_board].iter().enumerate() {
        for (item_index, item) in row.iter().enumerate() {
            if !check_boards[bingo_board][row_index][item_index] {
                score += item
            }
        }
    }

    score * bingo_number
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
    let (caller, boards) = parse_input(input);
    assert_eq!(part_one(&caller, &boards), 4512);
}

fn part_two(caller: &Vec<usize>, boards: &Vec<Board<usize>>) -> usize {
    let mut check_boards: Vec<Board<bool>> = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut bingo_boards: Vec<usize> = vec![0];

    let mut last_check_board = vec![vec![false; 5]; 5];
    let mut last_bingo_number = 0;
    let mut last_bingo_board = 0;

    for (call_index, call_number) in caller.iter().enumerate() {
        for (board_index, board) in boards.iter().enumerate() {
            for (row_index, row) in board.iter().enumerate() {
                for (item_index, item) in row.iter().enumerate() {
                    if item == call_number {
                        check_boards[board_index][row_index][item_index] = true;
                    }
                }
            }
        }

        if call_index >= 5 {
            for (board_index, board) in check_boards.iter().enumerate() {
                if bingo_boards.contains(&board_index) {
                    continue;
                }
                if is_bingo(&board) {
                    last_bingo_number = caller[call_index];
                    last_bingo_board = board_index;
                    last_check_board = check_boards[board_index].clone();
                    bingo_boards.push(last_bingo_board)
                }
            }
        }
    }

    let mut score = 0;
    for (row_index, row) in boards[last_bingo_board].iter().enumerate() {
        for (item_index, item) in row.iter().enumerate() {
            if !last_check_board[row_index][item_index] {
                score += item
            }
        }
    }

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
    let (caller, boards) = parse_input(input);
    assert_eq!(part_two(&caller, &boards), 1924);
}
