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

/// Extracts all signed integers (`i64`) from a byte slice (handles negative signs).
pub fn extract_numbers(bytes: &[u8]) -> Vec<i64> {
    let mut nums = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() || (b == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
        {
            let start = i;
            if b == b'-' {
                i += 1;
            }
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if let Some(n) = std::str::from_utf8(&bytes[start..i])
                .ok()
                .and_then(|s| s.parse::<i64>().ok())
            {
                nums.push(n);
            }
        } else {
            i += 1;
        }
    }
    nums
}

/// Extracts all values of type `T` from a string using `FromStr`.
pub fn extract_integers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    let mut result = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit()
            || (c == '-'
                && i + 1 < chars.len()
                && chars[i + 1].is_ascii_digit()
                && current.is_empty())
        {
            current.push(c);
        } else if !current.is_empty() {
            if let Ok(val) = current.parse::<T>() {
                result.push(val);
            }
            current.clear();
        }
        i += 1;
    }
    if let Ok(val) = current.parse::<T>() {
        result.push(val);
    }
    result
}

/// Greatest common divisor.
pub const fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Least common multiple.
pub const fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

pub fn transpose<T, R, Row, Out>(m: impl IntoIterator<Item = R>) -> Out
where
    R: IntoIterator<Item = T>,
    Row: FromIterator<T>,
    Out: FromIterator<Row>,
{
    let mut iters: Vec<R::IntoIter> = m.into_iter().map(IntoIterator::into_iter).collect();
    let mut cols: Vec<Row> = Vec::new();

    if !iters.is_empty() {
        'outer: loop {
            let mut col = Vec::with_capacity(iters.len());
            for it in &mut iters {
                match it.next() {
                    Some(v) => col.push(v),
                    None => {
                        assert!(col.is_empty(), "ragged matrix");
                        break 'outer;
                    }
                }
            }
            cols.push(col.into_iter().collect());
        }
    }
    cols.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers(b"x=12, y=-45, z: 999!"), vec![12, -45, 999]);
        assert_eq!(
            extract_numbers(b"no numbers here - and -"),
            Vec::<i64>::new()
        );
    }

    #[test]
    fn test_extract_integers() {
        let parsed: Vec<i32> = extract_integers("speed=-10, max=50, step=5");
        assert_eq!(parsed, vec![-10, 50, 5]);
    }

    #[test]
    fn test_gcd_and_lcm() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(4, 5), 20);
    }
}
