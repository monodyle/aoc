use std::fmt::Display;

pub enum Output {
	U32(u32),
	Str(String),
}

impl Display for Output {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Output::U32(u) => u.fmt(f),
			Output::Str(s) => s.fmt(f),
		}
	}
}
