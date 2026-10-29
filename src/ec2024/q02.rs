use crate::internal::lines;
use crate::{Part, Solver};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q02/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q02/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q02/3.txt"));

pub struct Q02;

impl Q02 {
    const WORDS_PREFIX: &[u8] = b"WORDS:";

    fn part1(input: &[u8]) -> usize {
        let mut it = lines(input);
        let words = Self::parse_words(it.next().expect("no words line"));

        it.map(|line| {
            words
                .iter()
                .map(|word| Self::count_occurrences(line, word))
                .sum::<usize>()
        })
        .sum()
    }

    fn part2(input: &[u8]) -> usize {
        let mut it = lines(input);
        let words = Self::parse_words_with_reverse(it.next().expect("no words line"));

        let mut mask: Vec<bool> = Vec::new();
        let mut total = 0;

        for line in it {
            mask.clear();
            mask.resize(line.len(), false);
            for w in &words {
                for (i, v) in line.windows(w.len()).enumerate() {
                    if v == w.as_slice() {
                        mask[i..i + w.len()].fill(true);
                    }
                }
            }
            total += mask.iter().filter(|&&b| b).count();
        }

        total
    }

    fn part3(input: &[u8]) -> usize {
        let mut it = lines(input);
        let words = Self::parse_words_with_reverse(it.next().expect("no words line"));
        let grid = it.filter(|l| !l.is_empty()).collect::<Vec<_>>();

        let rows = grid.len();
        let cols = grid.first().expect("empty grid").len();
        assert!(grid.iter().all(|l| l.len() == cols), "ragged grid");

        let mut mask = vec![false; rows * cols];
        for w in words {
            for r in 0..rows {
                for c in 0..cols {
                    if grid[r][c] != w[0] {
                        continue;
                    }

                    if (0..w.len()).all(|k| grid[r][(c + k) % cols] == w[k]) {
                        for k in 0..w.len() {
                            mask[r * cols + (c + k) % cols] = true;
                        }
                    }

                    if r + w.len() <= rows && (0..w.len()).all(|k| grid[r + k][c] == w[k]) {
                        for k in 0..w.len() {
                            mask[(r + k) * cols + c] = true;
                        }
                    }
                }
            }
        }

        mask.iter().filter(|&&b| b).count()
    }

    fn parse_words(s: &[u8]) -> Vec<&[u8]> {
        s.strip_prefix(Self::WORDS_PREFIX)
            .expect("words line must start with WORDS:")
            .split(|&c| c == b',')
            .filter(|w| !w.is_empty())
            .collect()
    }

    fn parse_words_with_reverse(s: &[u8]) -> Vec<Vec<u8>> {
        Self::parse_words(s)
            .iter()
            .flat_map(|&word| {
                let rev: Vec<_> = word.iter().rev().copied().collect();
                [word.to_vec(), rev]
            })
            .collect()
    }

    /// Counts (possibly overlapping) occurrences of `needle` in `haystack`.
    fn count_occurrences(haystack: &[u8], needle: &[u8]) -> usize {
        haystack
            .windows(needle.len())
            .filter(|&w| w == needle)
            .count()
    }
}

impl Solver for Q02 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn solve_from_input(&self, part: Part, input: &str) -> String {
        match part {
            Part::One => Self::part1(input.as_bytes()).to_string(),
            Part::Two => Self::part2(input.as_bytes()).to_string(),
            Part::Three => Self::part3(input.as_bytes()).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        4,
        "WORDS:THE,OWE,MES,ROD,HER

    AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE"
    )]
    #[case(
        3,
        "WORDS:THE,OWE,MES,ROD,HER

    THE FLAME SHIELDED THE HEART OF THE KINGS"
    )]
    #[case(
        2,
        "WORDS:THE,OWE,MES,ROD,HER

    OWE PO WER P OWE R"
    )]
    #[case(
        3,
        "WORDS:THE,OWE,MES,ROD,HER

    THERE IS THE END"
    )]
    fn part1(#[case] expected: usize, #[case] input: &str) {
        let result = Q02::part1(input.as_bytes());
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        42,
        "WORDS:THE,OWE,MES,ROD,HER,QAQ

    AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
    THE FLAME SHIELDED THE HEART OF THE KINGS
    POWE PO WER P OWE R
    THERE IS THE END
    QAQAQ"
    )]
    fn part2(#[case] expected: usize, #[case] input: &str) {
        let result = Q02::part2(input.as_bytes());
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        10,
        "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
    )]
    fn part3(#[case] expected: usize, #[case] input: &str) {
        let result = Q02::part3(input.as_bytes());
        assert_eq!(result, expected);
    }
}
