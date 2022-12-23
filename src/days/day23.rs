use std::collections::HashSet;

use itertools::{Itertools, MinMaxResult::MinMax};
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list0},
    IResult,
};

use crate::days::Day;

type Offset = (i64, i64);

/// North - South - West - East
const DIRS: [[Offset; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // NW, N, NE
    [(-1, 1), (0, 1), (1, 1)],    // SW, S, SE
    [(-1, -1), (-1, 0), (-1, 1)], // NW, W, SW
    [(1, -1), (1, 0), (1, 1)],    // NE, E, SE
];

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

/// alias for Point to convey there is an elf currently in this position
type ElfPosition = Point;

/// For fast searching, we keep the positions in a HashSet
type Elves = HashSet<ElfPosition>;

trait Searchable {
    fn get_elves_area(&self) -> usize;
    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool;
    fn has_elf_around(&self, x: i64, y: i64) -> bool;
    fn has_elf_on_side(&self, x: i64, y: i64, offsets: &[Offset; 3]) -> bool;
}

impl Searchable for Elves {
    /// Get the area occupied by the elves
    fn get_elves_area(&self) -> usize {
        let MinMax(min_x, max_x) = self.iter().minmax_by_key(|p| p.x) else {
            unimplemented!("Missing elves, no min and max found");
        };
        let MinMax(min_y, max_y) = self.iter().minmax_by_key(|p| p.y) else {
            unimplemented!("Missing elves, no min and max found");
        };
        ((max_x.x - min_x.x + 1) * (max_y.y - min_y.y + 1)) as usize
    }

    /// Check if there is an elf at x, y
    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool {
        self.contains(&ElfPosition { x, y })
    }

    /// Check if there is any elf around
    fn has_elf_around(&self, x: i64, y: i64) -> bool {
        DIRS.iter()
            .flatten()
            .any(|d| self.has_elf_at_pos(x + d.0, y + d.1))
    }

    /// Check if there is an elf in the top or bottom or left or right side
    ///
    /// offsets are the 3 cardinal directions of the considered side.
    /// e.g. for the top it would be : N, NW, NE
    fn has_elf_on_side(&self, x: i64, y: i64, offsets: &[Offset; 3]) -> bool {
        offsets
            .iter()
            .any(|d| self.has_elf_at_pos(x + d.0, y + d.1))
    }
}

/// Diffuse the elves, with their first considered direction being dir_counter (mod 4)
fn move_elves(elves: &mut Elves, dir_counter: usize) -> bool {
    // store the desired moves in a Vec (second element in the tuple is the elf's current position)
    let mut moves = Vec::<(Point, ElfPosition)>::new();
    // we want to check if any elf had the opportunity to move
    let mut has_moved = false;
    for elf in elves.iter() {
        // in case there are no elves around, the elf doesn't move
        if !elves.has_elf_around(elf.x, elf.y) {
            continue;
        }
        // try each of the 4 directions, starting with dir_counter (mod 4)
        for i in 0..4 {
            let dirs = DIRS[(dir_counter + i) % 4];
            // check if there are any elves in that direction
            if !elves.has_elf_on_side(elf.x, elf.y, &dirs) {
                // only if there are no elves in the 3 tiles on that side, we
                // propose a move at elf.pos.x + dirs[1].0, elf.pos.y + dirs[1].1
                let next = Point {
                    x: elf.x + dirs[1].0,
                    y: elf.y + dirs[1].1,
                };
                moves.push((next, elf.clone()));
                break;
            }
        }
    }
    // we need to only consider moves where there will be no collisions, so we dedup the moves by their target coord.
    // dedup function below expects equal items to be consecutive, hence the sort.
    let moves = moves
        .into_iter()
        .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
        .dedup_by_with_count(|a, b| a.0 == b.0)
        .filter(|(count, _)| *count == 1) // only keep non-collision moves
        .map(|(_, m)| m);
    for (next, elf) in moves {
        // move the elf
        elves.remove(&elf);
        elves.insert(next);
        has_moved = true;
    }
    // return if any elf was moved
    has_moved
}

pub struct Day23;

impl Day for Day23 {
    type Input = Elves;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, elements) = separated_list0(line_ending, many1(one_of(".#")))(input)?;
        let mut elves = Elves::new();
        for (y, row) in elements.iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if elem == &'#' {
                    elves.insert(ElfPosition {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            }
        }
        Ok((rest, elves))
    }

    type Output1 = usize;

    /// Part 1 took 4.1408ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // let's clone the elves to get a mutable version
        let mut elves = input.clone();
        // 10 rounds of diffusion
        for dir_counter in 0..10 {
            move_elves(&mut elves, dir_counter);
        }
        // get the number of free positions
        elves.get_elves_area() - elves.len()
    }

    type Output2 = usize;

    /// Part 2 took 454.4375ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // let's clone the elves to get a mutable version
        let mut elves = input.clone();
        // we iterate until no more elves move
        let mut dir_counter = 0;
        while move_elves(&mut elves, dir_counter) {
            dir_counter += 1;
        }
        // return the number (starts at 1) of the first round where no elf moved
        dir_counter + 1
    }
}
