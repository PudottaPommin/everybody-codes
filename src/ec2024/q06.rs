use crate::internal::{Part, Solver, lines};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q06/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q06/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q06/3.txt"));
use std::collections::HashMap;

pub struct Q06;

impl Q06 {
    const ROOT: &str = "RR";
    const FRUIT: &str = "@";
    const PESTS: [&str; 2] = ["BUG", "ANT"];

    /// Returns (child -> parent, branches holding a fruit). Lines named in `skip` are ignored.
    fn parse<'a>(input: &'a [u8], skip: &[&str]) -> (HashMap<&'a str, &'a str>, Vec<&'a str>) {
        let mut parent = HashMap::new();
        let mut holders = Vec::new();
        for line in lines(input) {
            let line = std::str::from_utf8(line).expect("input must be UTF-8");
            let (name, kids) = line.split_once(':').expect("line must contain ':'");
            if skip.contains(&name) {
                continue;
            }
            for kid in kids.split(',') {
                if kid == Self::FRUIT {
                    holders.push(name);
                } else {
                    parent.insert(kid, name);
                }
            }
        }
        (parent, holders)
    }

    /// Root-to-holder path, or None if it never reaches the root.
    fn path<'a>(parent: &HashMap<&'a str, &'a str>, holder: &'a str) -> Option<Vec<&'a str>> {
        let mut path = vec![holder];
        let mut cur = holder;
        while cur != Self::ROOT {
            cur = parent.get(cur).copied()?;
            path.push(cur);
            assert!(path.len() <= parent.len() + 1, "cycle through {cur}");
        }
        path.reverse();
        Some(path)
    }

    fn best(input: &[u8], skip: &[&str], abbreviate: bool) -> String {
        let (parent, holders) = Self::parse(input, skip);
        let paths: Vec<Vec<&str>> = holders
            .iter()
            .copied()
            .filter_map(|h| Self::path(&parent, h))
            .collect();

        let mut counts: HashMap<usize, usize> = HashMap::new();
        for p in &paths {
            *counts.entry(p.len()).or_default() += 1;
        }
        let best = paths
            .iter()
            .find(|p| counts[&p.len()] == 1)
            .expect("no fruit with a unique path length");

        let mut out: String = if abbreviate {
            best.iter().filter_map(|b| b.chars().next()).collect()
        } else {
            best.concat()
        };
        out.push_str(Self::FRUIT);
        out
    }
}

impl Solver for Q06 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn part1(&self, input: &str) -> String {
        Self::best(input.as_bytes(), &[], false)
    }
    fn part2(&self, input: &str) -> String {
        Self::best(input.as_bytes(), &[], true)
    }
    fn part3(&self, input: &str) -> String {
        Self::best(input.as_bytes(), &Self::PESTS, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RRB@",
        "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
    )]
    fn test_part1(#[case] expected: &str, #[case] input: &str) {
        assert_eq!(Q06.part1(input), expected);
    }

    #[rstest]
    #[case(
        "RB@",
        "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@"
    )]
    fn test_part2(#[case] expected: &str, #[case] input: &str) {
        assert_eq!(Q06.part2(input), expected);
    }
}
