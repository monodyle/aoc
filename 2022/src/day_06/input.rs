use super::Input;

pub fn read(value: String) -> Input {
    value.trim().chars().collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parser() {
		let raw = fs::read_to_string("./src/day_06/example.txt").unwrap();
        let input = read(raw);
        let expect = vec![
            'm', 'j', 'q', 'j', 'p', 'q', 'm', 'g', 'b', 'l', 'j', 's', 'p', 'h', 'd', 'z', 't',
            'n', 'v', 'j', 'f', 'q', 'w', 'r', 'c', 'g', 's', 'm', 'l', 'b',
        ];
        assert_eq!(input, expect)
    }
}
