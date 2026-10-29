use crate::{Part, Solver};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q01/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q01/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q01/3.txt"));

pub struct Q01;

impl Q01 {
    fn count_enemies(chunk_size: usize, input: &str) -> usize {
        input
            .as_bytes()
            .chunks(chunk_size)
            .map(|chunk| {
                let (n, sum) = chunk
                    .iter()
                    .filter_map(|&b| Enemy::try_from_byte(b)?.potion_cost())
                    .fold((0usize, 0usize), |(n, sum), cost| {
                        (n + 1, sum + cost as usize)
                    });

                sum + n * n.saturating_sub(1)
            })
            .sum()
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
    fn solve_from_input(&self, part: Part, input: &str) -> String {
        let chunk_size = match part {
            Part::One => 1,
            Part::Two => 2,
            Part::Three => 3,
        };
        Self::count_enemies(chunk_size, input).to_string()
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Enemy {
    Unknown,
    Ant,
    Beetle,
    Cockroach,
    Dragonfly,
}

impl Enemy {
    fn try_from_byte(value: u8) -> Option<Self> {
        match value {
            b'x' => Some(Enemy::Unknown),
            b'A' => Some(Enemy::Ant),
            b'B' => Some(Enemy::Beetle),
            b'C' => Some(Enemy::Cockroach),
            b'D' => Some(Enemy::Dragonfly),
            _ => None,
        }
    }

    fn potion_cost(&self) -> Option<u8> {
        match self {
            Enemy::Ant => Some(0),
            Enemy::Beetle => Some(1),
            Enemy::Cockroach => Some(3),
            Enemy::Dragonfly => Some(5),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 5, "ABBAC")]
    #[case(2, 28, "AxBCDDCAxD")]
    #[case(3, 30, "xBxAAABCDxCC")]
    fn part1(#[case] chunk_size: usize, #[case] expected: usize, #[case] input: &str) {
        let result = Q01::count_enemies(chunk_size, input);
        assert_eq!(result, expected);
    }
}
