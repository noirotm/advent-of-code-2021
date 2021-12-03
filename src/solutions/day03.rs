use crate::solver::{ReadExt, Solver};
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
        let bits = input[0].len();
        let mut ones = vec![0; bits];

        // count 1's for all input
        for s in input {
            for (i, b) in s.chars().enumerate() {
                if b == '1' {
                    ones[i] += 1;
                }
            }
        }

        let mut gamma = String::new();
        let mut epsilon = String::new();

        for n in ones {
            if n > input.len() / 2 {
                // we have more 1 than 0
                gamma.push('1');
                epsilon.push('0');
            } else {
                // more 0 than 1
                gamma.push('0');
                epsilon.push('1');
            }
        }

        let gamma = u64::from_str_radix(&gamma, 2).unwrap_or_default();
        let epsilon = u64::from_str_radix(&epsilon, 2).unwrap_or_default();

        gamma * epsilon
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let bits = input[0].len();

        let mut oxygen = input.clone();
        for bit in 0..bits {
            oxygen = filter_by_most_common(&oxygen, bit);
            if oxygen.len() == 1 {
                break;
            }
        }
        let oxygen = u64::from_str_radix(&oxygen[0], 2).unwrap_or_default();

        let mut co2 = input.clone();
        for bit in 0..bits {
            co2 = filter_by_least_common(&co2, bit);
            if co2.len() == 1 {
                break;
            }
        }
        let co2 = u64::from_str_radix(&co2[0], 2).unwrap_or_default();

        oxygen * co2
    }
}

fn filter_by_most_common(numbers: &[String], bit: usize) -> Vec<String> {
    let (zeroes, ones) = zeroes_and_ones(numbers, bit);
    let most_common = if ones >= zeroes { b'1' } else { b'0' };

    filter_by(numbers, bit, most_common)
}

fn filter_by_least_common(numbers: &[String], bit: usize) -> Vec<String> {
    let (zeroes, ones) = zeroes_and_ones(numbers, bit);
    let least_common = if ones >= zeroes { b'0' } else { b'1' };

    filter_by(numbers, bit, least_common)
}

fn zeroes_and_ones(numbers: &[String], bit: usize) -> (usize, usize) {
    let mut ones = 0;
    let mut zeroes = 0;

    // count 1's for all input
    for s in numbers {
        if let Some(&c) = s.as_bytes().get(bit) {
            if c == b'1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }
    }

    (zeroes, ones)
}

fn filter_by(numbers: &[String], bit: usize, val: u8) -> Vec<String> {
    numbers
        .iter()
        .filter(|n| n.as_bytes().get(bit).cloned().unwrap_or_default() == val)
        .cloned()
        .collect()
}
