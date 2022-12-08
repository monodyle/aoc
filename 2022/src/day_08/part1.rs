use aoc2022lib::Output;

use super::Input;

pub fn solve(input: &Input) -> Output {
    let mut count = (input.len() as u32 + input[0].len() as u32) * 2 - 4;

    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            let h = input[y][x];
            if input[..y].iter().all(|v| v[x] < h)
                || input[y][..x].iter().all(|&v| v < h)
                || input[y + 1..].iter().all(|v| v[x] < h)
                || input[y][x + 1..].iter().all(|&v| v < h)
            {
                count += 1
            }
        }
    }

    Output::U32(count)
}
