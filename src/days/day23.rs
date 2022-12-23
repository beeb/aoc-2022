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
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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

type ElvesPos = HashMap<Point, usize>;

trait Searchable {
    fn get_elves_bounds(&self) -> (Point, Point);
    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool;
    fn has_elf_around(&self, x: i64, y: i64) -> bool;
    fn has_elf_on_side(&self, x: i64, y: i64, offsets: &[Offset; 3]) -> bool;
}

impl Searchable for ElvesPos {
    fn get_elves_bounds(&self) -> (Point, Point) {
        let MinMax(min_x, max_x) = self.keys().minmax_by_key(|p| p.x) else {
            unimplemented!("Missing elves, no min and max found");
        };
        let MinMax(min_y, max_y) = self.keys().minmax_by_key(|p| p.y) else {
            unimplemented!("Missing elves, no min and max found");
        };
        (
            Point {
                x: min_x.x,
                y: min_y.y,
            },
            Point {
                x: max_x.x,
                y: max_y.y,
            },
        )
    }

    fn has_elf_at_pos(&self, x: i64, y: i64) -> bool {
        self.contains_key(&Point { x, y })
    }

    fn has_elf_around(&self, x: i64, y: i64) -> bool {
        DIRS.iter()
            .flatten()
            .any(|d| self.has_elf_at_pos(x + d.0, y + d.1))
    }

    fn has_elf_on_side(&self, x: i64, y: i64, offsets: &[Offset; 3]) -> bool {
        offsets
            .iter()
            .any(|d| self.has_elf_at_pos(x + d.0, y + d.1))
    }
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut elves = input.clone();
        let mut elves_pos = HashMap::<Point, usize>::new();
        for (idx, elf) in elves.iter().enumerate() {
            elves_pos.insert(elf.pos.clone(), idx);
        }
        for dir_counter in 0..10 {
            let mut moves = Vec::<(Point, usize)>::new();
            for (idx, elf) in elves.iter().enumerate() {
                if !elves_pos.has_elf_around(elf.pos.x, elf.pos.y) {
                    continue;
                }
                for i in 0..4 {
                    let dirs = DIRS[(dir_counter + i) % 4];
                    if !elves_pos.has_elf_on_side(elf.pos.x, elf.pos.y, &dirs) {
                        // propose move at elf.pos.x + dirs[1].0, elf.pos.y + dirs[1].1
                        let next = Point {
                            x: elf.pos.x + dirs[1].0,
                            y: elf.pos.y + dirs[1].1,
                        };
                        moves.push((next, idx));
                        break;
                    }
                }
            }
            let moves = moves.into_iter().dedup_by_with_count(|a, b| a.0 == b.0);
            for (count, (next, elf_idx)) in moves {
                if count > 1 {
                    continue;
                }
                let elf = elves.get_mut(elf_idx).unwrap();
                elves_pos.remove(&elf.pos);
                elves_pos.insert(next.clone(), elf_idx);
                elf.pos = next;
            }
        }
        let (top_left, bottom_right) = elves_pos.get_elves_bounds();
        ((bottom_right.x - top_left.x + 1) * (bottom_right.y - top_left.y + 1)) as usize
            - elves.len()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
