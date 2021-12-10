use crate::solver::{ReadExt, Solver};
use std::collections::VecDeque;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .flat_map(|s| find_first_incorrect(s))
            .flat_map(score)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut scores = input
            .iter()
            .filter(|s| find_first_incorrect(s).is_none())
            .map(|s| auto_complete(s))
            .map(|l| line_score(&l))
            .collect::<Vec<_>>();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn score(c: char) -> Option<u64> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}

fn score2(c: char) -> Option<u64> {
    match c {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}

fn line_score(l: &[char]) -> u64 {
    let mut score = 0;
    for &c in l {
        score *= 5;
        score += score2(c).unwrap_or_default();
    }
    score
}

fn auto_complete(s: &str) -> Vec<char> {
    let mut queue = VecDeque::new();

    for c in s.chars() {
        match c {
            '(' => queue.push_back(')'),
            '[' => queue.push_back(']'),
            '{' => queue.push_back('}'),
            '<' => queue.push_back('>'),
            _ => {
                let _ = queue.pop_back();
            }
        }
    }

    queue.into_iter().rev().collect()
}

fn find_first_incorrect(s: &str) -> Option<char> {
    let mut queue = VecDeque::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => queue.push_back(c),
            ')' => {
                if let Some(d) = queue.pop_back() {
                    if d != '(' {
                        return Some(c);
                    }
                }
            }
            ']' => {
                if let Some(d) = queue.pop_back() {
                    if d != '[' {
                        return Some(c);
                    }
                }
            }
            '}' => {
                if let Some(d) = queue.pop_back() {
                    if d != '{' {
                        return Some(c);
                    }
                }
            }
            '>' => {
                if let Some(d) = queue.pop_back() {
                    if d != '<' {
                        return Some(c);
                    }
                }
            }
            c => return Some(c),
        }
    }

    None
}
