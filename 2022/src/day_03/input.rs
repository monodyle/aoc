use std::fs;

use super::Input;

pub fn read() -> Input {
	let file = fs::read_to_string("./src/day_03/input.txt").unwrap();
	file.trim().lines().map(|f| f.to_owned()).collect()
}

/* #[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parser() {
		let input = read();
		assert_eq!(
			input,
			vec![
				"vJrwpWtwJgWrhcsFMMfFFhFp".to_owned(),
				"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned(),
				"PmmdzqPrVvPwwTWBwg".to_owned(),
				"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
				"ttgJtRGJQctTZtZT".to_owned(),
				"CrZsJsPPZsGzwwsLwLmpwMDw".to_owned()
			]
		);
	}
} */
