use std::fmt::Display;

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

pub trait LinesToUInt {
    fn to_vec_uint(&self) -> Vec<usize>;
}

impl LinesToUInt for String {
    fn to_vec_uint(&self) -> Vec<usize> {
        self.lines().map(|l| l.trim().parse().unwrap()).collect()
    }
}
