use super::Input;

pub fn read(input: String) -> Input {
	input.trim().split('\n').map(parse_round).collect()
}

fn parse_round(value: &str) -> (String, String) {
	let value = value.split_once(' ').unwrap();
	(value.0.to_string(), value.1.to_string())
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::*;

	#[test]
	fn test_parser() {
		let file = fs::read_to_string("./src/day_02/example.txt").unwrap();
		let input = read(file);
		assert_eq!(
			input,
			vec![
				("A".to_owned(), "Y".to_owned()),
				("B".to_owned(), "X".to_owned()),
				("C".to_owned(), "Z".to_owned())
			]
		);
	}
}
