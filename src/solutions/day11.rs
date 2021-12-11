use crate::grid::Grid;
use crate::solver::Solver;
use std::collections::{HashSet, VecDeque};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader_callback(r, |b| format!("{}", (b as char)).parse())
            .expect("can't read grid")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut input = input.clone();
        let mut total = 0;
        for _ in 0..100 {
            total += next_turn(&mut input);
        }

        total
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut input = input.clone();

        for i in 1..usize::MAX {
            let n = next_turn(&mut input);
            if n == 100 {
                return i;
            }
        }

        unreachable!()
    }
}

fn next_turn(g: &mut Grid<u8>) -> usize {
    // set of flashes that already happened
    let mut flashed = HashSet::new();

    // queue of points to analyze for possible flashes
    let mut queue = VecDeque::new();

    // 1. increment all
    for o in g.as_slice_mut() {
        *o += 1;
    }

    // 2. check for flashes
    for y in 0..g.h {
        for x in 0..g.w {
            if let Some(&o) = g.get((x, y)) {
                if o > 9 {
                    flashed.insert((x, y));
                    for c in g.neighbours_coords8((x, y)) {
                        queue.push_back(c);
                    }
                }
            }
        }
    }

    // 3. rinse and repeat while the queue is not empty
    while let Some(c) = queue.pop_front() {
        // if the point has flashed already, ignore it
        if flashed.contains(&c) {
            continue;
        }

        if let Some(o) = g.get_mut(c) {
            *o += 1;
            if *o > 9 {
                flashed.insert(c);
                for c in g.neighbours_coords8(c) {
                    queue.push_back(c);
                }
            }
        }
    }

    // 4. find all flashed and set them to zero
    for &c in &flashed {
        if let Some(o) = g.get_mut(c) {
            *o = 0;
        }
    }

    flashed.len()
}
