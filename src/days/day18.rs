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

const GRID_SIZE: usize = 22;

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
                    x: (x + 1) as usize, // offset by one to make sure we have margin around the object
                    y: (y + 1) as usize,
                    z: (z + 1) as usize,
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
                // this will never be out of bounds because we added 1 voxel of margin in all directions
                let n = (*x as i8 + dir.0, *y as i8 + dir.1, *z as i8 + dir.2);
                if !vol[n.0 as usize][n.1 as usize][n.2 as usize] {
                    open_sides += 1;
                }
            }
        }
        open_sides
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        // Volume of droplet
        let mut vol = [[[false; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
        // Populate the array from the input data
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        // keeping track of sides visible from the outside
        let mut visible = 0;
        // memoization to keep track of flooded voxels
        let mut seen = HashSet::<(i8, i8, i8)>::new();
        // we seed the flooding at 0, 0, 0
        seen.insert((0, 0, 0));
        // stack for BFS
        let mut stack = VecDeque::<(i8, i8, i8)>::new();
        stack.push_back((0, 0, 0));
        // let's flood it
        while let Some(voxel) = stack.pop_front() {
            for dir in DIRS {
                // check all neighbours
                let n = (voxel.0 + dir.0, voxel.1 + dir.1, voxel.2 + dir.2);

                // we only consider the ones that are within bounds and not visited before
                if seen.contains(&n) || !in_bounds(n) {
                    continue;
                }

                // if the neighbouring voxel is lava, we count its face.
                // we will only count it once since we added the currently visited voxel to the "seen" list
                // other faces of that lava voxel will be counted at another time when it's reached from other
                // directions
                if vol[n.0 as usize][n.1 as usize][n.2 as usize] {
                    visible += 1;
                    continue;
                }
                // in case this neighbour was first visited now, let's flood it and stack it for later visit
                stack.push_back(n);
                seen.insert(n);
            }
        }
        visible
    }
}
