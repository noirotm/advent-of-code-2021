use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u32>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .windows(2)
            .map(|c| match c {
                [a, b] => a < b,
                _ => false,
            })
            .filter(|&b| b)
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .windows(3)
            .map(|s| s.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .map(|c| match c {
                [a, b] => a < b,
                _ => false,
            })
            .filter(|&b| b)
            .count()
    }
}
