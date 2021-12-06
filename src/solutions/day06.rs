use crate::solver::{ReadExt, Solver};
use std::io::Read;
use std::iter::repeat;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_commas()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        /*let n = predict_until_turn(input, 80);
        n[80 - 1]*/
        solve_for_turns(input, 80)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        /*let n = predict_until_turn(input, 256);
        n[256 - 1]*/
        solve_for_turns(input, 256)
    }
}

fn solve_for_turns(pop: &[u8], turns: usize) -> usize {
    let mut fishes = [0; 9];
    for &i in pop {
        fishes[i as usize] += 1;
    }
    for _ in 0..turns {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.into_iter().sum()
}

#[allow(unused)]
fn next_state(pop: &mut Vec<u8>) {
    let mut len = pop.len();
    for f in pop.iter_mut() {
        if *f == 0 {
            len += 1;
            *f = 6;
        } else {
            *f -= 1;
        }
    }

    pop.resize(len, 8);
}

#[allow(unused)]
fn predict_until_turn(pop: &[u8], last_turn: usize) -> Vec<usize> {
    // initial lookup-table with all existing fishes
    let mut numbers = repeat(pop.len()).take(last_turn).collect::<Vec<_>>();

    // browse initial population to add fishes for the next turn then 7 by 7
    for fish in pop {
        let turn_added = *fish as usize + 1;

        let mut inc = 0;
        for turn in turn_added..=last_turn {
            if (turn - turn_added) % 7 == 0 {
                inc += 1;
            }
            numbers[turn - 1] += inc;
        }
    }

    // for each turn, compare with the previous one whether new fishes have appeared,
    // if so, add new ones 9 turns later, then every 7th turn
    for turn in 2..=last_turn {
        let new = numbers[turn - 1] - numbers[turn - 2];

        // same number as before, nothing to do
        if new == 0 {
            continue;
        }

        let next_turn = turn + 9;
        if next_turn <= last_turn {
            // we have new fishes, add them again 9 turns later
            // then add them again every 7th turn
            let mut inc = 0;
            for turn in next_turn..=last_turn {
                if (turn - next_turn) % 7 == 0 {
                    inc += new;
                }
                numbers[turn - 1] += inc;
            }
        }
    }

    numbers
}
