//TODO log2(0)
pub const fn log2(val: usize) -> usize {
    if val <= 1 {
        0
    } else {
        1 + log2(val / 2)
    }
}
