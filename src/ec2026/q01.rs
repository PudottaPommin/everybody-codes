use crate::internal::{Part, Solver};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2026/q01/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2026/q01/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2026/q01/3.txt"));

pub struct Q01;

impl Q01 {
    pub fn part1(input: &str) -> usize {
        Q01::parse_formations(input).fold(0usize, |acc, x| {
            let mut i = 1;
            loop {
                if i * i >= x {
                    return acc + (i * i - x);
                }
                i += 1
            }
        })
    }

    pub fn part2(input: &str) -> usize {
        Q01::parse_formations(input).fold(0usize, |acc, x| {
            let mut i = 3;
            loop {
                let pines = i * i - (i - 2) * (i - 2);
                if pines >= x {
                    return acc + (pines - x);
                }
                i += 1
            }
        })
    }

    pub fn part3(input: &str) -> usize {
        Q01::parse_formations(input).fold(0usize, |acc, x| {
            let mut i = 3;
            loop {
                let pines = i * i - 1;
                if pines >= x {
                    return acc + (pines - x);
                }
                i += 2
            }
        })
    }

    fn parse_formations(input: &str) -> impl Iterator<Item = usize> {
        input
            .split(',')
            .map(|s| s.parse::<usize>().unwrap_or_default())
    }
}

impl Solver for Q01 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn part1(&self, input: &str) -> String {
        Self::part1(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::part2(input).to_string()
    }

    fn part3(&self, input: &str) -> String {
        Self::part3(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(11, "3,11,12,16,24,25")]
    fn test_part1(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(Q01::part1(input), expected);
    }

    #[rstest]
    #[case(9, "3,11,12,16,24,25")]
    fn test_part2(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(Q01::part2(input), expected);
    }

    #[rstest]
    #[case(61, "3,11,12,16,24,25")]
    fn test_part3(#[case] expected: usize, #[case] input: &str) {
        assert_eq!(Q01::part3(input), expected);
    }
}
