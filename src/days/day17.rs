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
    /// row index of the lowest part of the piece
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

    /// Consider the part of the grid superposed with our piece
    fn move_left(&mut self, grid: &[u8]) -> &mut Self {
        let grid = &grid[self.z..self.z + 4];
        // check that all rows of our piece would have no collisions if we move them
        let can_move = grid.iter().enumerate().all(|(i, row)| {
            let piece_row = self.data[i];
            // try to move the piece row left, then BIT_AND with the grid, there should be no intersection = all zeroes
            (piece_row << 1 & row).count_ones() == 0
        });
        if !can_move {
            return self;
        }
        // shift the piece left
        self.data.iter_mut().for_each(|r| {
            *r <<= 1;
        });
        self
    }

    /// Consider the part of the grid superposed with our piece
    fn move_right(&mut self, grid: &[u8]) -> &mut Self {
        let grid = &grid[self.z..self.z + 4];
        // check that all rows of our piece would have no collisions if we move them
        let can_move = grid.iter().enumerate().all(|(i, row)| {
            let piece_row = self.data[i];
            if piece_row.trailing_ones() > 0 {
                // we're already all the way to the right, we can't move right
                return false;
            }
            // try to move the piece row right, then BIT_AND with the grid, there should be no intersection = all zeroes
            (piece_row >> 1 & row).count_ones() == 0
        });
        if !can_move {
            return self;
        }
        // shift the piece right
        self.data.iter_mut().for_each(|r| {
            *r >>= 1;
        });
        self
    }

    /// The grid slice we consider comprises the one below the piece's `z`, and the bottom row of our piece.
    /// This should be enough to check for collisions, since pieces are increasing in width at most on their lowest
    /// two rows
    fn can_move_down(&self, grid: &[u8]) -> bool {
        let grid = &grid[self.z - 1..=self.z];
        grid.iter().enumerate().all(|(i, row)| {
            // simulate if we moved down (i.e. piece row 0 (at index z) is in same position as grid row z-1)
            let piece_row = self.data[i];
            // check for collisions with the grid row
            (piece_row & row).count_ones() == 0
        })
    }

    fn move_down(&mut self) -> &mut Self {
        self.z -= 1;
        self
    }

    /// The highest z position is the base position (`self.z`), plus the height of the piece, minus 1
    fn get_highest_z(&self) -> usize {
        let height = self
            .data
            .iter()
            .fold_while(0, |acc, row| {
                if row.count_ones() > 0 {
                    // as long as the part grid has `1` in the row, we keep counting the rows
                    Continue(acc + 1)
                } else {
                    // the row is all zeroes, we stop counting the rows
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
    grid: [u8; 64],
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

    /// Part 1 took 0.1092ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // we make an infinite iterator for the air pushing actions
        let mut push = input.iter().cycle();
        // we create a grid that should hold the full stack, drawing the wall on the left (1 means occupied)
        let mut grid = [0b1000_0000_u8; 10_000];
        // at index 0 is the grid floor
        grid[0] = 0b1111_1111;
        let mut highest_z = 0usize; // keep track of the stack height

        // consider 2022 pieces
        for i in 0..2022 {
            let kind = (i % 5) as u8; // we cycle through the piece types
            let mut piece = Piece::new(kind, highest_z + 4); // the piece is initially 4 above the top of the stack

            // we loop until the part stops moving
            loop {
                // apply the sideways push from the air
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
                // part cannot move down, we can save current position into the grid (exit loop)
                if !piece.can_move_down(&grid) {
                    break;
                }
                // part can move down, we proceed
                piece.move_down();
            }
            // update the highest position of the stack
            highest_z = piece.get_highest_z().max(highest_z);
            // mutate grid, adding our piece
            for (j, piece_row) in piece.data.into_iter().enumerate() {
                grid[piece.z + j] |= piece_row;
            }
        }
        highest_z
    }

    type Output2 = usize;

    /// Part 2 took 0.562ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // We want to be able to know where in the input sequence lies the current air push, hence enumerate()
        // before the cycle() (so the index also cycles)
        let mut push = input.iter().enumerate().cycle();
        let mut skipped = 0; // this variable will keep track of how much height we fast-forwarded

        // Init our grid like last time, but a bit taller so we have time to notice a repeating pattern
        let mut grid = [0b10000000_u8; 100_000];
        grid[0] = 0b11111111;
        let mut highest_z = 0_usize;
        // Keep a cache of the (stack height, piece index) for a given combination of:
        //  - current piece type
        //  - air push index in the input sequence
        //  - and state of the last few top rows in the grid (64)
        // If we get a cache hit, it means we are in a repeating pattern and we can extract the period
        let mut cache = HashMap::<Identifier, (usize, usize)>::new();
        let mut i = 0;
        // We should iterate over 1e12 pieces
        while i < 1_000_000_000_000_usize {
            let kind = (i % 5) as u8;
            let mut piece = Piece::new(kind, highest_z + 4);
            let mut last_push_idx; // this will be used for the cache key
            loop {
                let (push_idx, push_type) = push.next().unwrap(); // get the next air push in the sequence (and its idx)
                last_push_idx = push_idx; // save it in the parent scope
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
            for (j, piece_row) in piece.data.into_iter().enumerate() {
                grid[piece.z + j] |= piece_row;
            }

            // Now we have finished our processing for this piece.
            // Let's save the state in the cache (only when we have enough grid height to save, i.e. 64 rows).
            // We only need to do this once, so when skipped > 0, we will not need to store cache anymore.
            if highest_z > 64 && skipped == 0 {
                // The identifier is the state of the system, which will repeat itself eventually.
                // We save the pattern of the to 64 rows in the grid
                let identifier = Identifier {
                    piece_kind: kind,
                    push_idx: last_push_idx,
                    grid: grid[highest_z - 63..=highest_z]
                        .try_into()
                        .expect("slice with incorrect length"),
                };
                if let Some((prev_height, prev_piece_idx)) =
                    cache.insert(identifier, (highest_z, i))
                {
                    // We got a cache hit! Let's compare the two states to know the period.
                    let height_diff = highest_z - prev_height;
                    let piece_diff = i - prev_piece_idx;
                    // Now we know how many periods (cycles) we can skip without affecting the state/repetition
                    let skip_repeats = (1_000_000_000_000_usize - i) / piece_diff;
                    let skip_pieces = skip_repeats * piece_diff;
                    let skip_height = skip_repeats * height_diff;
                    // We fast-forward our piece counter
                    i += skip_pieces;
                    // We record how much height we skipped, which we will add to the final result to get the real value
                    skipped = skip_height;
                }
            }
            i += 1;
        }
        highest_z + skipped
    }
}
