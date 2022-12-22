use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of},
    combinator::map,
    multi::{count, many1, separated_list0},
    sequence::separated_pair,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub enum Tile {
    Out,
    Free,
    Wall,
}

#[derive(Debug)]
pub enum Instruction {
    Walk(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl From<&Dir> for isize {
    fn from(value: &Dir) -> Self {
        match value {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }
}

impl From<isize> for Dir {
    fn from(value: isize) -> Self {
        match value {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            _ => Dir::Up,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Player {
    fn perform(&mut self, instr: &Instruction, grid: &[Vec<Tile>]) {
        match instr {
            Instruction::RotateLeft => {
                self.dir = (isize::from(&self.dir) - 1).rem_euclid(4).into();
            }
            Instruction::RotateRight => {
                self.dir = (isize::from(&self.dir) + 1).rem_euclid(4).into();
            }
            Instruction::Walk(dist) => {
                let mut remaining = *dist;
                while remaining > 0 {
                    let (next_x, next_y, next_tile) = match &self.dir {
                        Dir::Right => {
                            let row = &grid[self.y];
                            let (next_x, next_tile) = row
                                .iter()
                                .enumerate()
                                .cycle()
                                .skip(self.x + 1)
                                .find(|(_, t)| !matches!(t, Tile::Out))
                                .unwrap();
                            (next_x, self.y, next_tile)
                        }
                        Dir::Down => {
                            let col = grid.iter().map_while(|row| row.get(self.x));
                            let (next_y, next_tile) = col
                                .enumerate()
                                .cycle()
                                .skip(self.y + 1)
                                .find(|(_, t)| !matches!(t, Tile::Out))
                                .unwrap();
                            (self.x, next_y, next_tile)
                        }
                        Dir::Left => {
                            let row = &grid[self.y];
                            let (next_x, next_tile) = row
                                .iter()
                                .enumerate()
                                .rev()
                                .cycle()
                                .skip(row.len() - self.x)
                                .find(|(_, t)| !matches!(t, Tile::Out))
                                .unwrap();
                            (next_x, self.y, next_tile)
                        }
                        Dir::Up => {
                            let col = grid.iter().map_while(|row| row.get(self.x)).collect_vec();
                            let (next_y, next_tile) = col
                                .iter()
                                .enumerate()
                                .rev()
                                .cycle()
                                .skip(col.len() - self.y)
                                .find(|(_, t)| !matches!(t, Tile::Out))
                                .unwrap();
                            (self.x, next_y, *next_tile)
                        }
                    };
                    match next_tile {
                        Tile::Free => {
                            self.x = next_x;
                            self.y = next_y;
                        }
                        Tile::Wall => {
                            break;
                        }
                        Tile::Out => {
                            unreachable!()
                        }
                    }
                    remaining -= 1;
                }
            }
        }
    }
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list0(
        line_ending,
        many1(map(one_of(" .#"), |c| match c {
            ' ' => Tile::Out,
            '.' => Tile::Free,
            '#' => Tile::Wall,
            _ => unimplemented!(),
        })),
    )(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(alt((digit1, tag("R"), tag("L"))), |c| match c {
        "R" => Instruction::RotateRight,
        "L" => Instruction::RotateLeft,
        dist => Instruction::Walk(dist.parse::<usize>().unwrap()),
    }))(input)
}

pub struct Day22;

impl Day for Day22 {
    type Input = (Vec<Vec<Tile>>, Vec<Instruction>);

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_pair(parse_grid, count(line_ending, 2), parse_sequence)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let grid = &input.0;
        let instr = &input.1;
        let mut player = Player {
            x: grid[0]
                .iter()
                .position(|t| matches!(t, Tile::Free))
                .unwrap(),
            y: 0,
            dir: Dir::Right,
        };
        for i in instr {
            player.perform(i, grid);
        }
        1000 * (player.y + 1) + 4 * (player.x + 1) + isize::from(&player.dir) as usize
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
