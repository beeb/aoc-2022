use std::collections::HashMap;

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

#[derive(Debug, Clone, Default)]
pub struct Elf {
    pub pos: Point,
}

impl Elf {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            pos: Point { x, y },
        }
    }
}

/// For fast searching, we keep the positions in a HashMap
type ElvesPos = HashMap<Point, usize>;

trait Searchable {
    fn get_elves_area(&self) -> usize;
    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool;
    fn has_elf_around(&self, x: i64, y: i64) -> bool;
    fn has_elf_on_side(&self, x: i64, y: i64, offsets: &[Offset; 3]) -> bool;
}

impl Searchable for ElvesPos {
    /// Get the area occupied by the elves
    fn get_elves_area(&self) -> usize {
        let MinMax(min_x, max_x) = self.keys().minmax_by_key(|p| p.x) else {
            unimplemented!("Missing elves, no min and max found");
        };
        let MinMax(min_y, max_y) = self.keys().minmax_by_key(|p| p.y) else {
            unimplemented!("Missing elves, no min and max found");
        };
        ((max_x.x - min_x.x + 1) * (max_y.y - min_y.y + 1)) as usize
    }

    /// Check if there is an elf at x, y
    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool {
        self.contains_key(&Point { x, y })
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
fn move_elves(elves: &mut [Elf], elves_pos: &mut ElvesPos, dir_counter: usize) -> bool {
    // store the desired moves in a Vec (second element in the tuple is the position of the elf in the elves Vec)
    let mut moves = Vec::<(Point, usize)>::new();
    // we want to check if any elf had the opportunity to move
    let mut has_moved = false;
    for (idx, elf) in elves.iter().enumerate() {
        // in case there are no elves around, the elf doesn't move
        if !elves_pos.has_elf_around(elf.pos.x, elf.pos.y) {
            continue;
        }
        // try each of the 4 directions, starting with dir_counter (mod 4)
        for i in 0..4 {
            let dirs = DIRS[(dir_counter + i) % 4];
            // check if there are any elves in that direction
            if !elves_pos.has_elf_on_side(elf.pos.x, elf.pos.y, &dirs) {
                // only if there are no elves in the 3 tiles on that side, we
                // propose a move at elf.pos.x + dirs[1].0, elf.pos.y + dirs[1].1
                let next = Point {
                    x: elf.pos.x + dirs[1].0,
                    y: elf.pos.y + dirs[1].1,
                };
                moves.push((next, idx));
                break;
            }
        }
    }
    // we need to only consider moves where there will be no collisions, so we dedup the moves by their target coord.
    // dedup function below expects equal items to be consecutive, hence the sort.
    let moves = moves
        .into_iter()
        .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
        .dedup_by_with_count(|a, b| a.0 == b.0);
    for (count, (next, elf_idx)) in moves {
        // only consider moves that are unique (no collisions)
        if count > 1 {
            continue;
        }
        // move the elf, updating the positions hashmap too
        let elf = elves.get_mut(elf_idx).unwrap();
        elves_pos.remove(&elf.pos);
        elves_pos.insert(next.clone(), elf_idx);
        elf.pos = next;
        has_moved = true;
    }
    // return if any elf was moved
    has_moved
}

pub struct Day23;

impl Day for Day23 {
    type Input = Vec<Elf>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, elements) = separated_list0(line_ending, many1(one_of(".#")))(input)?;
        let mut elves = Vec::<Elf>::new();
        for (y, row) in elements.iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                if elem == &'#' {
                    elves.push(Elf::new(x as i64, y as i64));
                }
            }
        }
        Ok((rest, elves))
    }

    type Output1 = usize;

    /// Part 1 took 4.4991ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // let's clone the elves to get a mutable version
        let mut elves = input.clone();
        // we keep track of the elf index in a hashmap with its position as the key (for searching)
        let mut elves_pos = ElvesPos::new();
        // populate the positions hashmap
        for (idx, elf) in elves.iter().enumerate() {
            elves_pos.insert(elf.pos.clone(), idx);
        }
        // 10 rounds of diffusion
        for dir_counter in 0..10 {
            move_elves(&mut elves, &mut elves_pos, dir_counter);
        }
        // get the number of free positions
        elves_pos.get_elves_area() - elves.len()
    }

    type Output2 = usize;

    /// Part 2 took 463.5234ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // let's clone the elves to get a mutable version
        let mut elves = input.clone();
        // we keep track of the elf index in a hashmap with its position as the key (for searching)
        let mut elves_pos = ElvesPos::new();
        // populate the positions hashmap
        for (idx, elf) in elves.iter().enumerate() {
            elves_pos.insert(elf.pos.clone(), idx);
        }
        // we iterate until no more elves move
        let mut dir_counter = 0;
        while move_elves(&mut elves, &mut elves_pos, dir_counter) {
            dir_counter += 1;
        }
        // return the number (starts at 1) of the first round where no elf moved
        dir_counter + 1
    }
}
