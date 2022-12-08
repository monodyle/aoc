use super::Input;

pub fn read(input: String) -> Input {
    input.trim().lines().map(|f| f.to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parser() {
        let file = fs::read_to_string("./src/day_03/example.txt").unwrap();
        let input = read(file);
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
}
