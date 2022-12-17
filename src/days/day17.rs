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
    fn move_left(&mut self, grid: &[u8; 10_000]) -> &mut Self {
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
    fn move_right(&mut self, grid: &[u8; 10_000]) -> &mut Self {
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
    fn can_move_down(&self, grid: &[u8; 10_000]) -> bool {
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let push = input.iter().cycle();
        // at index 0 is the grid floor
        let mut grid = [0b10000000u8; 10_000];
        grid[0] = 0b11111111;
        let highest_z = 0usize;
        for i in 0..2022 {
            let kind = (i % 5) as u8;
            let mut piece = Piece::new(kind, highest_z + 4);
            println!("{piece}\n");
        }
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
