use std::collections::{HashSet, VecDeque};

use nom::{
    character::complete::{char, line_ending, u8},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

const DIRS: [(i8, i8, i8); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

const GRID_SIZE: usize = 20;

#[derive(Debug)]
pub struct Voxel {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

fn in_bounds(c: (i8, i8, i8)) -> bool {
    c.0 >= 0
        && c.0 < GRID_SIZE as i8
        && c.1 >= 0
        && c.1 < GRID_SIZE as i8
        && c.2 >= 0
        && c.2 < GRID_SIZE as i8
}

pub struct Day18;

impl Day for Day18 {
    type Input = Vec<Voxel>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((u8, char(','), u8, char(','), u8)),
                |(x, _, y, _, z)| Voxel {
                    x: x as usize,
                    y: y as usize,
                    z: z as usize,
                },
            ),
        )(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.0259ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // Let's save the voxels into a 3D array
        let mut vol = [[[false; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
        let mut open_sides = 0;
        // Populate the array from the input data
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        // Let's check the open faces for each voxel (top, bottom, left, right, front, back)
        for Voxel { x, y, z } in input.iter() {
            for dir in DIRS {
                let n = (*x as i8 + dir.0, *y as i8 + dir.1, *z as i8 + dir.2);
                if !in_bounds(n) {
                    open_sides += 1;
                    continue;
                }
                if !vol[n.0 as usize][n.1 as usize][n.2 as usize] {
                    open_sides += 1;
                }
            }
        }
        open_sides
    }

    type Output2 = usize;

    /// Part 2 took 0.1277ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // Volume of solidified lava
        let mut vol = [[[false; 20]; 20]; 20];
        let mut flood = [[[false; 20]; 20]; 20];
        // Populate the array from the input data
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        let mut visible = 0;
        flood[0][0][0] = true;
        let mut seen = HashSet::<(i8, i8, i8)>::new();
        seen.insert((0, 0, 0));
        let mut stack = VecDeque::<(i8, i8, i8)>::new();
        stack.push_back((0, 0, 0));
        while let Some(voxel) = stack.pop_front() {
            for dir in DIRS {
                let n = (voxel.0 + dir.0, voxel.1 + dir.1, voxel.2 + dir.2);
                if !in_bounds(n) {
                    continue;
                }
                if vol[n.0 as usize][n.1 as usize][n.2 as usize] {
                    visible += 1;
                } else if !seen.contains(&n) {
                    flood[n.0 as usize][n.1 as usize][n.2 as usize] = true;
                    stack.push_back(n);
                    seen.insert(n);
                }
            }
        }
        /* for (y, sl) in flood[8].iter().enumerate() {
            for &fl in sl {
                print!("{}", fl as u8);
            }
            print!(" ");
            for v in vol[8][y] {
                print!("{}", v as u8);
            }
            println!();
        }  *//*
          visible = 0;
          for Voxel { x, y, z } in input.iter() {
              if *x == 19 || flood[*x + 1][*y][*z] {
                  visible += 1;
              }
              if *x == 0 || flood[*x - 1][*y][*z] {
                  visible += 1;
              }
              if *y == 19 || flood[*x][*y + 1][*z] {
                  visible += 1;
              }
              if *y == 0 || flood[*x][*y - 1][*z] {
                  visible += 1;
              }
              if *z == 19 || flood[*x][*y][*z + 1] {
                  visible += 1;
              }
              if *z == 0 || flood[*x][*y][*z - 1] {
                  visible += 1;
              }
          } */
        visible
    }
}
