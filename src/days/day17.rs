use std::collections::HashMap;

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use nom::{character::complete::anychar, combinator::map, multi::many1, IResult};

use crate::days::Day;

#[derive(Debug)]
pub enum Push {
    Left,
    Right,
}

pub struct Piece {
    /// piece shape and lateral placement, starting from the bottom
    data: [u8; 4],
    /// row containing the lowest part of the piece
    z: usize,
}

impl Piece {
    fn new(kind: u8, z: usize) -> Self {
        // bottom to top
        let data = match kind {
            0 => [0b11110, 0b0, 0b0, 0b0],
            1 => [0b1000, 0b11100, 0b1000, 0b0],
            2 => [0b11100, 0b100, 0b100, 0b0],
            3 => [0b10000; 4],
            4 => [0b11000, 0b11000, 0b0, 0b0],
            _ => unimplemented!("only 5 pieces types are available"),
        };
        Self { data, z }
    }

    /// the grid slice we consider is aligned with the piece data
    fn move_left(&mut self, grid: &[u8]) -> &mut Self {
        let grid = &grid[self.z..self.z + 4];
        let can_move = grid.iter().enumerate().all(|(i, row)| {
            let piece_row = self.data[i];
            (piece_row << 1 & row).count_ones() == 0
        });
        if !can_move {
            return self;
        }
        self.data.iter_mut().for_each(|r| {
            *r <<= 1;
        });
        self
    }

    /// the grid slice we get is aligned with the piece data
    fn move_right(&mut self, grid: &[u8]) -> &mut Self {
        let grid = &grid[self.z..self.z + 4];
        let can_move = grid.iter().enumerate().all(|(i, row)| {
            let piece_row = self.data[i];
            if piece_row.trailing_ones() > 0 {
                return false;
            }
            (piece_row >> 1 & row).count_ones() == 0
        });
        if !can_move {
            return self;
        }
        self.data.iter_mut().for_each(|r| {
            *r >>= 1;
        });
        self
    }

    /// the grid slice we consider comprises the one below the piece's z, and the bottom row of our piece
    /// this should be enough to check for collisions, since pieces are increasing in section only on their lowest
    /// two rows
    fn can_move_down(&self, grid: &[u8]) -> bool {
        let grid = &grid[self.z - 1..=self.z];
        grid.iter().enumerate().all(|(i, row)| {
            let piece_row = self.data[i]; // simulate if we moved down
            (piece_row & row).count_ones() == 0
        })
    }

    fn move_down(&mut self) -> &mut Self {
        self.z -= 1;
        self
    }

    fn get_highest_z(&self) -> usize {
        let height = self
            .data
            .iter()
            .fold_while(0, |acc, row| {
                if row.count_ones() > 0 {
                    Continue(acc + 1)
                } else {
                    Done(acc)
                }
            })
            .into_inner();
        self.z + height - 1
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:#09b}\n{:#09b}\n{:#09b}\n{:#09b}",
            self.data[3], self.data[2], self.data[1], self.data[0]
        )
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Identifier {
    piece_kind: u8,
    push_idx: usize,
    grid: [u8; 16],
}

pub struct Day17;

impl Day for Day17 {
    type Input = Vec<Push>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        many1(map(anychar, |c| match c {
            '<' => Push::Left,
            _ => Push::Right,
        }))(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.175ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut push = input.iter().cycle();
        // at index 0 is the grid floor
        let mut grid = [0b10000000u8; 10_000];
        grid[0] = 0b11111111;
        let mut highest_z = 0usize;
        for i in 0..2022 {
            let kind = (i % 5) as u8;
            let mut piece = Piece::new(kind, highest_z + 4);
            loop {
                match push.next() {
                    Some(Push::Left) => {
                        piece.move_left(&grid);
                    }
                    Some(Push::Right) => {
                        piece.move_right(&grid);
                    }
                    None => {
                        unreachable!("we cycle through the iterator indefinitely");
                    }
                }
                if !piece.can_move_down(&grid) {
                    break;
                }
                piece.move_down();
            }
            highest_z = piece.get_highest_z().max(highest_z);
            // mutate grid
            for (j, piece_row) in piece.data.into_iter().enumerate() {
                grid[piece.z + j] |= piece_row;
            }
        }
        highest_z
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut push = input.iter().enumerate().cycle();
        let mut skipped = 0;
        // at index 0 is the grid floor
        let mut grid = [0b10000000_u8; 100_000];
        grid[0] = 0b11111111;
        let mut highest_z = 0_usize;
        // keep a cache of the stack height and piece index for a given combination of current piece type, air push
        // index in the sequence, and state of the last few rows
        let mut cache = HashMap::<Identifier, (usize, usize)>::new();
        let mut i = 0;
        while i < 1_000_000_000_000_usize {
            let kind = (i % 5) as u8;
            let mut piece = Piece::new(kind, highest_z + 4);
            let mut last_push_idx;
            loop {
                let (push_idx, push_type) = push.next().unwrap();
                last_push_idx = push_idx;
                match push_type {
                    Push::Left => {
                        piece.move_left(&grid);
                    }
                    Push::Right => {
                        piece.move_right(&grid);
                    }
                }
                if !piece.can_move_down(&grid) {
                    break;
                }
                piece.move_down();
            }
            highest_z = piece.get_highest_z().max(highest_z);
            // mutate grid
            for (j, piece_row) in piece.data.into_iter().enumerate() {
                grid[piece.z + j] |= piece_row;
            }
            if highest_z > 15 && skipped == 0 {
                let identifier = Identifier {
                    piece_kind: kind,
                    push_idx: last_push_idx,
                    grid: grid[highest_z - 15..=highest_z]
                        .try_into()
                        .expect("slice with incorrect length"),
                };
                if let Some((prev_height, prev_piece_idx)) =
                    cache.insert(identifier, (highest_z, i))
                {
                    let height_diff = highest_z - prev_height;
                    let piece_diff = i - prev_piece_idx;
                    let skip_repeats = (1_000_000_000_000_usize - i) / piece_diff;
                    let skip_pieces = skip_repeats * piece_diff;
                    let skip_height = skip_repeats * height_diff;
                    i += skip_pieces;
                    skipped = skip_height;
                }
            }
            i += 1;
        }
        highest_z + skipped
    }
}
