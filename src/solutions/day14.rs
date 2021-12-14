use crate::solver::{ReadExt, Solver};
use std::collections::HashMap;
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Instr;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Instr::from_reader(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        solve_for_turns(input, 10)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        solve_for_turns(input, 40)
    }
}

fn solve_for_turns(input: &Instr, turns: usize) -> usize {
    let mut pairs_map =
        input
            .template
            .windows(2)
            .fold(HashMap::<&[char], usize>::new(), |mut e, c| {
                let entry = e.entry(c).or_default();
                *entry += 1;
                e
            });
    let mut char_map = input
        .template
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut e, &c| {
            let entry = e.entry(c).or_default();
            *entry += 1;
            e
        });

    for _ in 0..turns {
        // all existing pairs
        let pairs = pairs_map
            .iter()
            .map(|(c, n)| (c.to_vec(), *n))
            .collect::<Vec<_>>();

        // for each pair, generate the next ones, and increment
        pairs_map = HashMap::new();
        for (c, n) in pairs {
            if let Some((a, b)) = input.rules.get(&c) {
                // new pairs
                let entry = pairs_map.entry(a).or_default();
                *entry += n;
                let entry = pairs_map.entry(b).or_default();
                *entry += n;

                // total character amount
                let new = a[1];
                let entry = char_map.entry(new).or_default();
                *entry += n;
            }
        }
    }

    let max = char_map.values().max().unwrap_or(&0);
    let min = char_map.values().min().unwrap_or(&0);

    *max - *min
}

type RuleMap = HashMap<Vec<char>, (Vec<char>, Vec<char>)>;

#[derive(Debug)]
pub struct Instr {
    template: Vec<char>,
    rules: RuleMap,
}

impl Instr {
    fn from_reader<R: Read>(r: R) -> Self {
        let s: Vec<String> = r.split_groups();
        let template = s[0].chars().collect();
        let rules_str: Vec<String> = s[1].as_bytes().split_lines();

        let mut rules = HashMap::new();
        for r in &rules_str {
            let chars = r.chars().collect::<Vec<_>>();
            let key = chars[0..2].to_vec();
            let val = chars[6];
            rules.insert(key, (vec![chars[0], val], vec![val, chars[1]]));
        }

        Self { template, rules }
    }
}
