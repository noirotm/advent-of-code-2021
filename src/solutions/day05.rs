use crate::grid::{Coord, Grid};
use crate::solver::{ReadExt, Solver};
use std::cmp::Ordering;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Line>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        r.split_lines()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let lines = input
            .iter()
            .filter(|l| l.is_horizontal_or_vertical())
            .cloned()
            .collect::<Vec<_>>();

        make_grid(&lines)
            .as_slice()
            .iter()
            .filter(|&&c| c >= 2)
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        make_grid(input)
            .as_slice()
            .iter()
            .filter(|&&c| c >= 2)
            .count()
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    from: Pt,
    to: Pt,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    fn points(&self) -> Vec<Pt> {
        let mut points = vec![];

        let inc_x = match self.to.x.cmp(&self.from.x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let inc_y = match self.to.y.cmp(&self.from.y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };

        points.push(self.from.clone());

        let mut x = self.from.x as isize;
        let mut y = self.from.y as isize;

        while x as usize != self.to.x || y as usize != self.to.y {
            x += inc_x;
            y += inc_y;
            points.push(Pt {
                x: x as usize,
                y: y as usize,
            });
        }

        points
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" -> ");
        let from = s.next().ok_or("invalid string")?.parse()?;
        let to = s.next().ok_or("invalid string")?.parse()?;
        Ok(Self { from, to })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pt {
    x: usize,
    y: usize,
}

impl Coord for Pt {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl FromStr for Pt {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let x = s.next().ok_or("invalid string")?.parse()?;
        let y = s.next().ok_or("invalid string")?.parse()?;
        Ok(Self { x, y })
    }
}

fn make_grid(lines: &[Line]) -> Grid<usize> {
    let w = lines
        .iter()
        .flat_map(|p| [p.from.x, p.to.x])
        .max()
        .unwrap_or_default()
        + 1;
    let h = lines
        .iter()
        .flat_map(|p| [p.from.y, p.to.y])
        .max()
        .unwrap_or_default()
        + 1;
    let mut grid = Grid::new_with(w, h, 0usize);

    for line in lines {
        for pt in line.points() {
            if let Some(cell) = grid.get_mut(pt) {
                *cell += 1;
            }
        }
    }

    grid
}
