use crate::solver::{ReadExt, Solver};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        unimplemented!()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        unimplemented!()
    }
}
