use std::{
    collections::{btree_map::Entry, BTreeMap},
    fs,
};

use super::{Input, Procedure, Stacks};

pub fn read() -> Input {
    let file = fs::read_to_string("./src/day_05/input.txt").unwrap();
    let (stack, procedure) = file.trim_end().split_once("\n\n").unwrap();
    (parse_stack(stack), parse_procedure(procedure))
}

fn parse_stack(value: &str) -> Stacks {
    let mut stacks: Stacks = BTreeMap::new();

    for line in value.lines().collect::<Vec<_>>().split_last().unwrap().1 {
        for (i, c) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            let v = c[1];
            if v.eq(&' ') {
                continue;
            }
            match stacks.entry(i as u8 + 1) {
                Entry::Occupied(mut entry) => {
                    entry.insert(format!("{}{}", entry.get(), v));
                }
                Entry::Vacant(entry) => {
                    entry.insert(format!("{}", v));
                }
            }
        }
    }

    stacks
}

fn parse_procedure(value: &str) -> Vec<Procedure> {
    value
        .lines()
        .map(|l| l.trim().split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            Procedure::new(
                v[1].parse().unwrap(),
                v[3].parse().unwrap(),
                v[5].parse().unwrap(),
            )
        })
        .collect()
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = read();

        let mut stacks: Stacks = BTreeMap::new();
        stacks.insert(1, "NZ".to_owned());
        stacks.insert(2, "DCM".to_owned());
        stacks.insert(3, "P".to_owned());

        let mut rearrangement: Vec<Procedure> = Vec::new();
        rearrangement.push(Procedure::new(1, 2, 1));
        rearrangement.push(Procedure::new(3, 1, 3));
        rearrangement.push(Procedure::new(2, 2, 1));
        rearrangement.push(Procedure::new(1, 1, 2));

        assert_eq!(input, (stacks, rearrangement));
    }
} */
