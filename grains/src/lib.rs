pub fn square(s: u32) -> u64 {
    if s < 1 || 64 < s {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}
