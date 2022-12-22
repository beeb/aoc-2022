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

const CUBE_SIZE: usize = 50;

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
    pub fn perform(&mut self, instr: &Instruction, grid: &[Vec<Tile>], part2: bool) {
        match instr {
            Instruction::RotateLeft => {
                self.dir = (isize::from(&self.dir) - 1).rem_euclid(4).into();
            }
            Instruction::RotateRight => {
                self.dir = (isize::from(&self.dir) + 1).rem_euclid(4).into();
            }
            Instruction::Walk(dist) => {
                if part2 {
                    self.walk2(dist, grid);
                } else {
                    self.walk(dist, grid);
                }
            }
        }
    }

    fn get_face(x: usize, y: usize) -> usize {
        if y < CUBE_SIZE {
            if x < 2 * CUBE_SIZE {
                return 1;
            }
            return 2;
        }
        if y < 2 * CUBE_SIZE {
            return 3;
        }
        if y < 3 * CUBE_SIZE {
            if x < CUBE_SIZE {
                return 4;
            }
            return 5;
        }
        6
    }

    fn coord_in_face(x: usize, y: usize) -> (usize, usize) {
        let face = Self::get_face(x, y);
        match face {
            1 => (x - CUBE_SIZE, y),
            2 => (x - 2 * CUBE_SIZE, y),
            3 => (x - CUBE_SIZE, y - CUBE_SIZE),
            4 => (x, y - 2 * CUBE_SIZE),
            5 => (x - CUBE_SIZE, y - 2 * CUBE_SIZE),
            _ => (x, y - 3 * CUBE_SIZE),
        }
    }

    fn face_to_global(x: usize, y: usize, face: usize) -> (usize, usize) {
        match face {
            1 => (x + CUBE_SIZE, y),
            2 => (x + 2 * CUBE_SIZE, y),
            3 => (x + CUBE_SIZE, y + CUBE_SIZE),
            4 => (x, y + 2 * CUBE_SIZE),
            5 => (x + CUBE_SIZE, y + 2 * CUBE_SIZE),
            _ => (x, y + 3 * CUBE_SIZE),
        }
    }

    fn next_coord(x: usize, y: usize, dir: &Dir) -> (usize, usize, Dir) {
        let face = Self::get_face(x, y);
        let (xl, yl) = Self::coord_in_face(x, y);
        match face {
            1 => match dir {
                Dir::Right => (x + 1, y, Dir::Right),
                Dir::Left => {
                    if xl == 0 {
                        let (nx, ny) = Self::face_to_global(xl, CUBE_SIZE - yl - 1, 4);
                        (nx, ny, Dir::Right)
                    } else {
                        (x - 1, y, Dir::Left)
                    }
                }
                Dir::Up => {
                    if yl == 0 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 6);
                        (nx, ny, Dir::Right)
                    } else {
                        (x, y - 1, Dir::Up)
                    }
                }
                Dir::Down => (x, y + 1, Dir::Down),
            },
            2 => match dir {
                Dir::Right => {
                    if xl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(xl, CUBE_SIZE - yl - 1, 5);
                        (nx, ny, Dir::Left)
                    } else {
                        (x + 1, y, Dir::Right)
                    }
                }
                Dir::Left => (x - 1, y, Dir::Left),
                Dir::Up => {
                    if yl == 0 {
                        let (nx, ny) = Self::face_to_global(xl, CUBE_SIZE - 1, 6);
                        (nx, ny, Dir::Up)
                    } else {
                        (x, y - 1, Dir::Up)
                    }
                }
                Dir::Down => {
                    if yl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 3);
                        (nx, ny, Dir::Left)
                    } else {
                        (x, y + 1, Dir::Down)
                    }
                }
            },
            3 => match dir {
                Dir::Right => {
                    if xl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 2);
                        (nx, ny, Dir::Up)
                    } else {
                        (x + 1, y, Dir::Right)
                    }
                }
                Dir::Left => {
                    if xl == 0 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 4);
                        (nx, ny, Dir::Down)
                    } else {
                        (x - 1, y, Dir::Left)
                    }
                }
                Dir::Up => (x, y - 1, Dir::Up),
                Dir::Down => (x, y + 1, Dir::Down),
            },
            4 => match dir {
                Dir::Right => (x + 1, y, Dir::Right),
                Dir::Left => {
                    if xl == 0 {
                        let (nx, ny) = Self::face_to_global(xl, CUBE_SIZE - yl - 1, 1);
                        (nx, ny, Dir::Right)
                    } else {
                        (x - 1, y, Dir::Left)
                    }
                }
                Dir::Up => {
                    if yl == 0 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 3);
                        (nx, ny, Dir::Right)
                    } else {
                        (x, y - 1, Dir::Up)
                    }
                }
                Dir::Down => (x, y + 1, Dir::Down),
            },
            5 => match dir {
                Dir::Right => {
                    if xl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(xl, CUBE_SIZE - yl - 1, 2);
                        (nx, ny, Dir::Left)
                    } else {
                        (x + 1, y, Dir::Right)
                    }
                }
                Dir::Left => (x - 1, y, Dir::Left),
                Dir::Up => (x, y - 1, Dir::Up),
                Dir::Down => {
                    if yl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 6);
                        (nx, ny, Dir::Left)
                    } else {
                        (x, y + 1, Dir::Down)
                    }
                }
            },
            _ => match dir {
                Dir::Right => {
                    if xl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 5);
                        (nx, ny, Dir::Up)
                    } else {
                        (x + 1, y, Dir::Right)
                    }
                }
                Dir::Left => {
                    if xl == 0 {
                        let (nx, ny) = Self::face_to_global(yl, xl, 1);
                        (nx, ny, Dir::Down)
                    } else {
                        (x - 1, y, Dir::Left)
                    }
                }
                Dir::Up => (x, y - 1, Dir::Up),
                Dir::Down => {
                    if yl == CUBE_SIZE - 1 {
                        let (nx, ny) = Self::face_to_global(xl, yl, 2);
                        (nx, ny, Dir::Down)
                    } else {
                        (x, y + 1, Dir::Down)
                    }
                }
            },
        }
    }

    fn walk2(&mut self, dist: &usize, grid: &[Vec<Tile>]) {
        let mut remaining = *dist;
        while remaining > 0 {
            let (next_x, next_y, next_dir) = Self::next_coord(self.x, self.y, &self.dir);
            let next_tile = &grid[next_y][next_x];
            match next_tile {
                Tile::Free => {
                    self.x = next_x;
                    self.y = next_y;
                    self.dir = next_dir;
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

    fn walk(&mut self, dist: &usize, grid: &[Vec<Tile>]) {
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
            player.perform(i, grid, false);
        }
        1000 * (player.y + 1) + 4 * (player.x + 1) + isize::from(&player.dir) as usize
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
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
            player.perform(i, grid, true);
        }
        1000 * (player.y + 1) + 4 * (player.x + 1) + isize::from(&player.dir) as usize
    }
}
