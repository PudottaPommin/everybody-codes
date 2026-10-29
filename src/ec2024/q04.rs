use crate::internal::{Part, Solver, digits_to_usize, lines, median};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q04/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q04/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q04/3.txt"));

pub struct Q04;

impl Q04 {
    fn part(input: &[u8]) -> usize {
        let mut min = usize::MAX;
        let nails = lines(input)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let d = digits_to_usize(l).unwrap();
                if d < min {
                    min = d;
                }
                d
            })
            .collect::<Vec<_>>();
        nails.iter().fold(0, |acc, &nail| {
            if nail != min {
                acc + nail.abs_diff(min)
            } else {
                acc
            }
        })
    }

    fn part3(input: &[u8]) -> usize {
        let mut min: Option<usize> = None;
        let mut max: Option<usize> = None;
        let mut nails = lines(input)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let d = digits_to_usize(l).unwrap();
                match min {
                    Some(m) => {
                        if d < m {
                            min = Some(d);
                        }
                    }
                    None => min = Some(d),
                }
                match max {
                    Some(m) => {
                        if d > m {
                            max = Some(d);
                        }
                    }
                    None => max = Some(d),
                }

                d
            })
            .collect::<Vec<_>>();

        let med = median(&mut nails);

        nails.iter().fold(0, |acc, &nail| {
            if med != nail {
                acc + med.abs_diff(nail)
            } else {
                acc
            }
        })
    }
}

impl Solver for Q04 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn solve_from_input(&self, part: Part, input: &str) -> String {
        match part {
            Part::Three => Self::part3(input.as_bytes()).to_string(),
            _ => Self::part(input.as_bytes()).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        10, "3
4
7
8"
    )]
    fn part1(#[case] expected: usize, #[case] input: &str) {
        let result = Q04::part(input.as_bytes());
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(
        8,
        "2
4
5
6
8"
    )]
    fn part3(#[case] expected: usize, #[case] input: &str) {
        let result = Q04::part3(input.as_bytes());
        assert_eq!(result, expected)
    }
}
