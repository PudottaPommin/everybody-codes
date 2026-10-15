pub fn digits_to_usize(slice: &[u8]) -> Option<usize> {
    let mut n: usize = 0;
    for &b in slice {
        if !b.is_ascii_digit() {
            return None;
        }
        n = n * 10 + (b - b'0') as usize;
    }
    Some(n)
}

pub fn median(n: &mut [usize]) -> usize {
    let mid = n.len() / 2;
    *n.select_nth_unstable(mid).1
}
