use nom::{character::complete::anychar, combinator::map, multi::many1, IResult};

use crate::days::Day;

#[derive(Debug)]
pub enum Push {
    Left,
    Right,
}

pub struct Piece {
    data: [u8; 4],
    z: usize,
}

impl Piece {
    fn new(kind: u8, z: usize) -> Self {
        let data = match kind {
            0 => [0b0, 0b0, 0b0, 0b11110],
            1 => [0b0, 0b1000, 0b11100, 0b1000],
            2 => [0b0, 0b100, 0b100, 0b11100],
            3 => [0b10000; 4],
            4 => [0b0, 0b0, 0b11000, 0b11000],
            _ => unimplemented!("only 5 pieces types are available"),
        };
        Self { data, z }
    }

    fn move_left(&mut self, grid: [u8; 4]) -> &mut Self {
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

    fn move_right(&mut self, grid: [u8; 4]) -> &mut Self {
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
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:#09b}\n{:#09b}\n{:#09b}\n{:#09b}",
            self.data[0], self.data[1], self.data[2], self.data[3]
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
        let grid = [0b10000000u8; 10_000];
        let highest_z = 0usize;
        for i in 0..2022 {
            let kind = (i % 5) as u8;
            let mut piece = Piece::new(kind, highest_z + 4);
        }
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
