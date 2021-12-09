use crate::grid::Grid;
use crate::solver::Solver;
use std::collections::{HashSet, VecDeque};
use std::io::Read;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<u8>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader_callback(r, |b| format!("{}", (b as char)).parse())
            .expect("can't read grid")
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        find_low_points(input)
            .into_iter()
            .flat_map(|c| input.get(c))
            .map(|&v| v as u64 + 1)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        // find all low points, then
        // for each low point, do a flood fill in the basin
        let mut sizes = find_low_points(input)
            .into_iter()
            .map(|c| flood_fill_basin(input, c))
            .collect::<Vec<_>>();

        sizes.sort_unstable();

        sizes.into_iter().rev().take(3).fold(1, |a, b| a * b as u64)
    }
}

fn find_low_points(g: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for y in 0..g.h {
        for x in 0..g.w {
            if let Some(&val) = g.get((x, y)) {
                let is_low = g.neighbours4((x, y)).iter().all(|&&n| n > val);
                if is_low {
                    v.push((x, y));
                }
            }
        }
    }
    v
}

fn flood_fill_basin(g: &Grid<u8>, orig: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    let mut basin_size = 0;

    to_visit.push_back(orig);
    while let Some(pt) = to_visit.pop_front() {
        if let Some(&val) = g.get(pt) {
            if val != 9 && !visited.contains(&pt) {
                basin_size += 1;
                for c in neighbours4_coords(pt) {
                    to_visit.push_back(c);
                }
            }
        }
        visited.insert(pt);
    }

    basin_size
}

fn neighbours4_coords((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .iter()
        .map(|&(dx, dy)| (((x as isize) + dx), ((y as isize) + dy)))
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
