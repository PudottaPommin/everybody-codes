use std::fmt::Display;
use std::io;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Part {
    One = 1,
    Two = 2,
    Three = 3,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl Part {
    pub const ALL: [Part; 3] = [Part::One, Part::Two, Part::Three];

    pub fn from_number(n: u8) -> Option<Part> {
        Part::ALL.into_iter().find(|&p| p as u8 == n)
    }
}

pub trait Solver {
    fn input(&self, _part: Part) -> Option<&'static str> {
        None
    }

    fn part1(&self, _input: &str) -> String {
        String::from("<unsolved>")
    }

    fn part2(&self, _input: &str) -> String {
        String::from("<unsolved>")
    }

    fn part3(&self, _input: &str) -> String {
        String::from("<unsolved>")
    }

    fn solve_from_input(&self, part: Part, input: &str) -> String {
        match part {
            Part::One => self.part1(input),
            Part::Two => self.part2(input),
            Part::Three => self.part3(input),
        }
    }

    fn solve_timed(&self, part: Part, input: &str) -> (String, std::time::Duration) {
        let start = std::time::Instant::now();
        let answer = self.solve_from_input(part, input);
        let elapsed = start.elapsed();
        (answer, elapsed)
    }

    fn solve_from_path(
        &self,
        part: Part,
        input_path: &str,
    ) -> Option<(String, std::time::Duration)> {
        let input = match read_input(input_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("[P{part:}]: skipped ('{input_path}': {e})");
                return None;
            }
        };
        let (answer, elapsed) = self.solve_timed(part, &input);
        println!("[P{part:}]: {answer} in {:?}", elapsed);
        Some((answer, elapsed))
    }

    fn solve_from_paths(&self, input_paths: &[&str]) {
        assert_eq!(
            input_paths.len(),
            Part::ALL.len(),
            "expected {} input paths, got {}",
            Part::ALL.len(),
            input_paths.len()
        );
        for (part, path) in Part::ALL.into_iter().zip(input_paths) {
            self.solve_from_path(part, path);
        }
    }
}

pub fn read_input(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}
pub fn lines(input: &[u8]) -> impl Iterator<Item = &[u8]> {
    input
        .split(|&b| b == b'\n')
        .map(|l| l.trim_ascii())
        .filter(|l| !l.is_empty())
}

/// Splits a line on anything that isn't part of a number, then parses each token.
pub fn parse_numbers<T: std::str::FromStr>(line: &[u8]) -> impl Iterator<Item = T> {
    line.split(|&b| !(b.is_ascii_digit() || b == b'-' || b == b'.'))
        .filter(|t| !t.is_empty())
        .map(|t| {
            let s = std::str::from_utf8(t).unwrap(); // only ASCII bytes by construction
            s.parse::<T>()
                .unwrap_or_else(|_| panic!("cannot parse token {s:?}"))
        })
}

pub fn input_path(year: u16, quest: u8, part: Part) -> String {
    format!("inputs/ec{year}/q{quest:02}/{}.txt", part as u8)
}
