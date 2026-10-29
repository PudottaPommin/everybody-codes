use crate::internal::{Part, Solver, lines, parse_numbers, transpose};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q05/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q05/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q05/3.txt"));
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct Q05;

impl Q05 {
    pub fn part1(input: &[u8]) -> usize {
        let mut numbers: VecDeque<VecDeque<usize>> = transpose(lines(input).map(parse_numbers));
        assert!(!numbers.is_empty(), "no columns parsed from input");

        for round in 0..10 {
            Self::dance_round(&mut numbers, round);
        }
        Self::shout(&numbers)
    }

    pub fn part2(input: &[u8]) -> usize {
        let mut numbers: VecDeque<VecDeque<usize>> = transpose(lines(input).map(parse_numbers));
        assert!(!numbers.is_empty(), "no columns parsed from input");

        let mut shouts: FxHashMap<usize, usize> =
            FxHashMap::with_capacity_and_hasher(1 << 16, Default::default());
        for round in 0.. {
            Self::dance_round(&mut numbers, round);

            let number = Self::shout(&numbers);
            let count = shouts.entry(number).or_insert(0);
            *count += 1;
            if *count == 2024 {
                return number * (round + 1);
            }
        }
        unreachable!()
    }

    pub fn part3(input: &[u8]) -> usize {
        let mut numbers: VecDeque<VecDeque<usize>> = transpose(lines(input).map(parse_numbers));
        assert!(!numbers.is_empty(), "no columns parsed from input");
        let n = numbers.len();

        let mut seen: FxHashSet<usize> =
            FxHashSet::with_capacity_and_hasher(1 << 16, Default::default());
        let mut best = 0;

        for round in 0.. {
            // The state before this round: grid + which column is the clapper's.
            if !seen.insert(Self::state_hash(&numbers, round % n)) {
                break; // cycle closed, nothing new can be shouted
            }

            Self::dance_round(&mut numbers, round);
            let shouted = Self::shout(&numbers);
            best = best.max(shouted);
        }

        best
    }

    fn state_hash(numbers: &VecDeque<VecDeque<usize>>, turn: usize) -> usize {
        let mut h = DefaultHasher::new();
        numbers.hash(&mut h);
        turn.hash(&mut h);
        h.finish() as usize
    }

    fn shout(numbers: &VecDeque<VecDeque<usize>>) -> usize {
        numbers.iter().fold(0usize, |acc, col| {
            let v = *col.front().expect("empty column");
            let digits = v.checked_ilog10().map_or(1, |d| d + 1);
            acc * 10usize.pow(digits) + v
        })
    }

    fn dance_round(numbers: &mut VecDeque<VecDeque<usize>>, round: usize) {
        let n = numbers.len();
        let from = round % n;
        let to = (from + 1) % n;

        let clapper = numbers[from].pop_front().expect("no clapper");
        assert!(clapper > 0, "clap count must be positive");

        let col = &mut numbers[to];
        let len = col.len();
        let pos = (clapper - 1) % (2 * len);
        let idx = if pos < len { pos } else { 2 * len - pos };
        col.insert(idx, clapper);
    }
}

impl Solver for Q05 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn part1(&self, input: &str) -> String {
        Self::part1(input.as_bytes()).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::part2(input.as_bytes()).to_string()
    }

    fn part3(&self, input: &str) -> String {
        Self::part3(input.as_bytes()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    #[case(
        2323,
        "2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4"
    )]
    fn test_part1(#[case] expected: usize, #[case] input: &str) {
        let result = Q05::part1(input.as_bytes());
        assert_eq!(result, expected);
    }

    #[test_log::test(rstest)]
    #[case(
        50877075,
        "2 3 4 5
6 7 8 9"
    )]
    fn test_part2(#[case] expected: usize, #[case] input: &str) {
        let result = Q05::part2(input.as_bytes());
        assert_eq!(result, expected);
    }

    #[test_log::test(rstest)]
    #[case(
        6584,
        "2 3 4 5
6 7 8 9"
    )]
    fn test_part3(#[case] expected: usize, #[case] input: &str) {
        let result = Q05::part3(input.as_bytes());
        assert_eq!(result, expected);
    }
}
