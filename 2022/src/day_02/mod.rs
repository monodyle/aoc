use aoc2022lib::Output;

pub mod input;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	pub fn from_str(s: &str) -> Self {
		match s {
			"A" | "X" => Self::Rock,
			"B" | "Y" => Self::Paper,
			"C" | "Z" => Self::Scissors,
			_ => unreachable!(),
		}
	}

	pub fn to_score(self) -> u32 {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}

	pub fn fight(&self, opponent: &Move) -> Result {
		if self == opponent {
			return Result::Tie;
		}
		if (self == &Self::Rock && opponent == &Self::Scissors)
			|| (self == &Self::Scissors && opponent == &Self::Paper)
			|| (self == &Self::Paper && opponent == &Self::Rock)
		{
			return Result::Win;
		}
		Result::Lose
	}

	pub fn derive(&self, win: bool) -> Self {
		let (to_win, to_lose) = if self == &Self::Rock {
			(Self::Paper, Self::Scissors)
		} else if self == &Self::Paper {
			(Self::Scissors, Self::Rock)
		} else {
			(Self::Rock, Self::Paper)
		};
		if win {
			to_win
		} else {
			to_lose
		}
	}
}

#[derive(Debug)]
pub enum Result {
	Win,
	Lose,
	Tie,
}

impl Result {
	pub fn to_score(&self) -> u32 {
		match self {
			Result::Win => 6,
			Result::Lose => 0,
			Result::Tie => 3,
		}
	}
}

pub type Input = Vec<(String, String)>;

pub fn solve() -> (Output, Output) {
	let input = input::read();
	(part1::solve(&input), part2::solve(&input))
}
