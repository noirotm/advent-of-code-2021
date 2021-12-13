use crate::grid::{Grid, GridPoint};
use crate::solver::{ReadExt, Solver};
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Instr;
    type Output1 = usize;
    type Output2 = &'static str;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let g: Vec<String> = r.split_groups();
        let points = g[0].as_bytes().split_lines();
        let folds = g[1].as_bytes().split_lines();

        Instr { points, folds }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let g = make_grid(&input.points);
        let f = input.folds.first().expect("non empty folds");

        let g = fold_grid(&g, f);

        g.as_slice()
            .iter()
            .filter(|p| matches!(p, Paper::Dot))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut g = make_grid(&input.points);
        for f in &input.folds {
            g = fold_grid(&g, f);
        }

        println!("{}", g);

        // output without OCR :)
        "HECRZKPR"
    }
}

fn make_grid(dots: &[GridPoint]) -> Grid<Paper> {
    let mut w = 0;
    let mut h = 0;
    for d in dots {
        if d.x + 1 > w {
            w = d.x + 1;
        }
        if d.y + 1 > h {
            h = d.y + 1;
        }
    }

    let mut g = Grid::new_with(w, h, Paper::Empty);
    for d in dots {
        if let Some(p) = g.get_mut(d) {
            *p = Paper::Dot;
        }
    }

    g
}

fn fold_grid(g: &Grid<Paper>, f: &Fold) -> Grid<Paper> {
    match *f {
        Fold::X(x) => fold_grid_x(g, x),
        Fold::Y(y) => fold_grid_y(g, y),
    }
}

fn fold_grid_x(g: &Grid<Paper>, fold_x: usize) -> Grid<Paper> {
    let mut new_grid = Grid::new_with(fold_x, g.h, Paper::Empty);

    // copy old points
    for y in 0..g.h {
        for x in 0..fold_x {
            let p = g.get((x, y)).unwrap_or(&Paper::Empty);
            if let Some(np) = new_grid.get_mut((x, y)) {
                if matches!(p, Paper::Dot) {
                    *np = Paper::Dot;
                }
            }
        }
    }

    // copy folded points
    for y in 0..g.h {
        for x in (fold_x + 1)..g.w {
            let new_x = fold_x - abs_diff(fold_x, x);
            if matches!(g.get((x, y)), Some(Paper::Dot)) {
                if let Some(np) = new_grid.get_mut((new_x, y)) {
                    *np = Paper::Dot;
                }
            }
        }
    }

    new_grid
}

fn fold_grid_y(g: &Grid<Paper>, fold_y: usize) -> Grid<Paper> {
    let mut new_grid = Grid::new_with(g.w, fold_y, Paper::Empty);

    // copy old points
    for y in 0..fold_y {
        for x in 0..g.w {
            let p = g.get((x, y)).unwrap_or(&Paper::Empty);
            if let Some(np) = new_grid.get_mut((x, y)) {
                if matches!(p, Paper::Dot) {
                    *np = Paper::Dot;
                }
            }
        }
    }

    // copy folded points
    for y in (fold_y + 1)..g.h {
        for x in 0..g.w {
            let new_y = fold_y - abs_diff(fold_y, y);
            if matches!(g.get((x, y)), Some(Paper::Dot)) {
                if let Some(np) = new_grid.get_mut((x, new_y)) {
                    *np = Paper::Dot;
                }
            }
        }
    }

    new_grid
}

fn abs_diff(a: usize, b: usize) -> usize {
    max(a, b) - min(a, b)
}

#[derive(Debug)]
pub struct Instr {
    points: Vec<GridPoint>,
    folds: Vec<Fold>,
}

impl FromStr for GridPoint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        Ok(Self {
            x: s.next()
                .ok_or("invalid string")?
                .parse()
                .map_err(|_| "invalid number")?,
            y: s.next()
                .ok_or("invalid string")?
                .parse()
                .map_err(|_| "invalid number")?,
        })
    }
}

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[11..];
        let mut s = s.split('=');
        let axis = s.next().ok_or("invalid string")?;
        let val = s
            .next()
            .ok_or("invalid string")?
            .parse()
            .map_err(|_| "invalid number")?;
        match axis {
            "x" => Ok(Fold::X(val)),
            "y" => Ok(Fold::Y(val)),
            _ => Err("invalid axis"),
        }
    }
}

#[derive(Clone)]
enum Paper {
    Empty,
    Dot,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Paper::Empty => ".",
                Paper::Dot => "#",
            }
        )
    }
}
