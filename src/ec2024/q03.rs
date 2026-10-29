use crate::internal::{Part, Solver, grid::*, lines};

const INPUT_P1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q03/1.txt"));
const INPUT_P2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q03/2.txt"));
const INPUT_P3: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/ec2024/q03/3.txt"));

pub struct Q03;

impl Q03 {
    fn part(part: Part, input: &[u8]) -> usize {
        let mut depth: Grid<usize> = Grid::from_rows(
            lines(input)
                .filter(|l| !l.is_empty())
                .map(|l| l.iter().map(|&b| usize::from(b == b'#'))),
        );
        let (w, h) = (depth.width(), depth.height());

        loop {
            let next = Grid::from_fn(w, h, |p| {
                let d = depth[p];
                if d == 0 {
                    return 0;
                }

                let ok = match part {
                    Part::Three => {
                        Self::all_at_least(&depth, p, d, &Pos::CROSS_MATRIX)
                            && Self::all_at_least(&depth, p, d, &Pos::DIAGONAL_MATRIX)
                    }
                    _ => Self::all_at_least(&depth, p, d, &Pos::CROSS_MATRIX),
                };

                if ok { d + 1 } else { d }
            });

            if next == depth {
                break;
            }
            depth = next;
        }

        depth.iter().sum()
    }

    fn all_at_least(grid: &Grid<usize>, p: Pos, d: usize, offsets: &[(isize, isize)]) -> bool {
        offsets.iter().all(|&(dx, dy)| {
            p.new_relative(dx, dy)
                .and_then(|n| grid.get(n))
                .copied()
                .unwrap_or(0)
                >= d
        })
    }
}

impl Solver for Q03 {
    fn input(&self, part: Part) -> Option<&'static str> {
        match part {
            Part::One => Some(INPUT_P1),
            Part::Two => Some(INPUT_P2),
            Part::Three => Some(INPUT_P3),
        }
    }
    fn solve_from_input(&self, part: Part, input: &str) -> String {
        Self::part(part, input.as_bytes()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        35,
        "..........
..###.##..
...####...
..######..
..######..
...####...
.........."
    )]
    fn part1(#[case] expected: usize, #[case] input: &str) {
        let result = Q03::part(Part::One, input.as_bytes());
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(
        29,
        "..........
..###.##..
...####...
..######..
..######..
...####...
.........."
    )]
    fn part3(#[case] expected: usize, #[case] input: &str) {
        let result = Q03::part(Part::Three, input.as_bytes());
        assert_eq!(result, expected)
    }
}
