use crate::solver::{ReadExt, Solver};
use std::cmp::{max, min};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_commas()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let pos = median(input).round() as u64;
        cost_for_position(input, pos)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (min, max) = min_max(input);
        (min..=max)
            .into_iter()
            .map(|pos| cost_for_position2(input, pos))
            .min()
            .unwrap_or_default()
    }
}

fn median(a: &[u64]) -> f64 {
    let mut a = a.to_vec();
    a.sort_unstable();

    if a.len() % 2 == 0 {
        (a[a.len() / 2 - 1] + a[a.len() / 2]) as f64 / 2.0
    } else {
        a[a.len() / 2] as f64
    }
}

fn min_max(a: &[u64]) -> (u64, u64) {
    let (mut min, mut max) = (0, 0);
    for &i in a {
        if i < min {
            min = i;
        }
        if i > max {
            max = i;
        }
    }
    (min, max)
}

fn cost_for_position(n: &[u64], pos: u64) -> u64 {
    n.iter().map(|&n| abs_diff(n, pos)).sum()
}

fn abs_diff(a: u64, b: u64) -> u64 {
    max(a, b) - min(a, b)
}

fn cost_for_position2(n: &[u64], pos: u64) -> u64 {
    n.iter().map(|&n| cost(n, pos)).sum()
}

fn cost(from: u64, to: u64) -> u64 {
    let n = abs_diff(from, to);
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_works() {
        assert_eq!(cost(1, 2), 1);
        assert_eq!(cost(2, 3), 1);
        assert_eq!(cost(1, 3), 3);
        assert_eq!(cost(1, 4), 6);
    }
}
