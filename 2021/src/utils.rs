use std::{fmt::Display, fs};

pub enum Solution {
    Int(isize),
    UInt(usize),
    // Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Int(i) => i.fmt(f),
            Solution::UInt(u) => u.fmt(f),
            // Solution::Str(s) => s.fmt(f),
        }
    }
}

pub fn load_input(day: &str) -> String {
    let mut path = "input/day".to_owned();
    path.push_str(day);
    fs::read_to_string(path).unwrap()
}
