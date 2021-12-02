pub trait LinesToVec32 {
    fn to_vec32(&self) -> Vec<u32>;
}

impl LinesToVec32 for String {
    fn to_vec32(&self) -> Vec<u32> {
        self.lines().map(|l| l.trim().parse().unwrap()).collect()
    }
}
