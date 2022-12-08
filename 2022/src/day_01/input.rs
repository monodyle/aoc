use super::Input;

pub fn read(input: String) -> Input {
	input.trim().split("\n\n").map(parse_calories).collect()
}

fn parse_calories(value: &str) -> u32 {
	value.lines().flat_map(|l| l.parse::<u32>()).sum::<u32>()
}
