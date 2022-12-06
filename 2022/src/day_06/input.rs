use std::fs;

use super::Input;

pub fn read() -> Input {
    let file = fs::read_to_string("./src/day_06/input.txt").unwrap();
    file.trim().chars().collect()
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = read();
        let expect = vec![
            'm', 'j', 'q', 'j', 'p', 'q', 'm', 'g', 'b', 'l', 'j', 's', 'p', 'h', 'd', 'z', 't',
            'n', 'v', 'j', 'f', 'q', 'w', 'r', 'c', 'g', 's', 'm', 'l', 'b',
        ];
        assert_eq!(input, expect)
    }
} */
