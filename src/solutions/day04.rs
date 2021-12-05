use crate::grid::Grid;
use crate::solver::{ReadExt, Solver};
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Bingo;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, mut r: R) -> Self::Input {
        let s: Vec<String> = r.split_groups();
        let picks = s[0].as_bytes().split_commas();
        let boards = s[1..]
            .iter()
            .flat_map(|s| Grid::from_split_whitespace_reader(s.as_bytes()))
            .collect();

        Bingo { picks, boards }
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let mut boards = input.boards.clone();

        for &pick in &input.picks {
            for board in &mut boards {
                mark_pick(board, pick);

                if has_bingo(board) {
                    return pick as u64 * board_score(board);
                }
            }
        }

        unreachable!()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let mut boards = input.boards.clone();
        let mut boards_remaining = input.boards.len();

        for &pick in &input.picks {
            for board in &mut boards {
                if has_bingo(board) {
                    continue;
                }

                mark_pick(board, pick);

                if has_bingo(board) {
                    boards_remaining -= 1;
                }

                if boards_remaining == 0 {
                    return pick as u64 * board_score(board);
                }
            }
        }

        unreachable!()
    }
}

pub struct Bingo {
    picks: Vec<u8>,
    boards: Vec<Grid<BingoCell>>,
}

#[derive(Clone)]
pub struct BingoCell {
    value: u8,
    marked: bool,
}

impl FromStr for BingoCell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse()?,
            marked: false,
        })
    }
}

fn mark_pick(board: &mut Grid<BingoCell>, n: u8) {
    if let Some(c) = board.as_slice_mut().iter_mut().find(|c| c.value == n) {
        c.marked = true;
    }
}

fn has_bingo(board: &Grid<BingoCell>) -> bool {
    for row in 0..board.h {
        if board.iter_row(row).all(|c| c.marked) {
            return true;
        }
    }
    for col in 0..board.w {
        if board.iter_col(col).all(|c| c.marked) {
            return true;
        }
    }

    false
}

fn board_score(board: &Grid<BingoCell>) -> u64 {
    board
        .as_slice()
        .iter()
        .filter(|c| !c.marked)
        .map(|c| c.value as u64)
        .sum::<u64>()
}
