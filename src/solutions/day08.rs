use crate::solver::{ReadExt, Solver};
use std::collections::{BTreeSet, HashMap};
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Entry>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let unique_lengths = [2, 3, 4, 7];

        input
            .iter()
            .flat_map(|e| e.result.iter())
            .filter(|s| unique_lengths.contains(&s.len()))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().flat_map(|e| solve_entry(e)).sum()
    }
}

pub type Word = BTreeSet<u8>;

pub struct Entry {
    patterns: Vec<Word>,
    result: Vec<Word>,
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" | ");
        let patterns = s
            .next()
            .ok_or("invalid input")?
            .split_whitespace()
            .map(|s| BTreeSet::from_iter(s.as_bytes().iter().cloned()))
            .collect();
        let result = s
            .next()
            .ok_or("invalid input")?
            .split_whitespace()
            .map(|s| BTreeSet::from_iter(s.as_bytes().iter().cloned()))
            .collect();

        Ok(Self { patterns, result })
    }
}

fn solve_entry(e: &Entry) -> Option<usize> {
    let one = e.patterns.iter().find(|p| p.len() == 2)?;
    let seven = e.patterns.iter().find(|p| p.len() == 3)?;
    let four = e.patterns.iter().find(|p| p.len() == 4)?;
    let eight = e.patterns.iter().find(|p| p.len() == 7)?;

    let mut solution = HashMap::new();
    solution.insert(one, 1);
    solution.insert(four, 4);
    solution.insert(seven, 7);
    solution.insert(eight, 8);

    for pattern in e.patterns.iter() {
        if pattern.len() == 5 {
            solution.insert(
                pattern,
                if pattern.intersection(one).count() == 2 {
                    3
                } else if pattern.intersection(four).count() == 3 {
                    5
                } else {
                    2
                },
            );
        } else if pattern.len() == 6 {
            solution.insert(
                pattern,
                if pattern.intersection(one).count() == 1 {
                    6
                } else if pattern.intersection(four).count() == 4 {
                    9
                } else {
                    0
                },
            );
        }
    }

    let n = e
        .result
        .iter()
        .flat_map(|s| solution.get(s))
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .ok()?;

    Some(n)
}
