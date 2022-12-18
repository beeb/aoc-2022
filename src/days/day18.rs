use nom::{
    character::complete::{char, line_ending, u8},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct Voxel {
    pub x: usize,
    pub y: usize,
    pub z: usize,
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut vol = [[[false; 20]; 20]; 20];
        let mut open_sides = 0;
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        for Voxel { x, y, z } in input.iter() {
            if *x == 19 || !vol[*x + 1][*y][*z] {
                open_sides += 1;
            }
            if *x == 0 || !vol[*x - 1][*y][*z] {
                open_sides += 1;
            }
            if *y == 19 || !vol[*x][*y + 1][*z] {
                open_sides += 1;
            }
            if *y == 0 || !vol[*x][*y - 1][*z] {
                open_sides += 1;
            }
            if *z == 19 || !vol[*x][*y][*z + 1] {
                open_sides += 1;
            }
            if *z == 0 || !vol[*x][*y][*z - 1] {
                open_sides += 1;
            }
        }
        open_sides
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut vol = [[[false; 20]; 20]; 20];
        let mut flood = [[[false; 20]; 20]; 20];
        flood[0][0][0] = true; // seed the flooding algorithm
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        let mut prev_visible = 0;
        let mut visible;
        loop {
            // flood
            for x in 0..20 {
                for y in 0..20 {
                    for z in 0..20 {
                        if vol[x][y][z] || flood[x][y][z] {
                            // if we have a solid here, it's obviously not part of the ouside
                            // if we already are flooded, we skip too
                            continue;
                        }
                        // check if any neighbor was already "flooded" (and is not solid)
                        if x < 19 && flood[x + 1][y][z] && !vol[x + 1][y][z] {
                            flood[x][y][z] = true;
                            continue;
                        }
                        if x > 0 && flood[x - 1][y][z] && !vol[x - 1][y][z] {
                            flood[x][y][z] = true;
                            continue;
                        }
                        if y < 19 && flood[x][y + 1][z] && !vol[x][y + 1][z] {
                            flood[x][y][z] = true;
                            continue;
                        }
                        if y > 0 && flood[x][y - 1][z] && !vol[x][y - 1][z] {
                            flood[x][y][z] = true;
                            continue;
                        }
                        if z < 19 && flood[x][y][z + 1] && !vol[x][y][z + 1] {
                            flood[x][y][z] = true;
                            continue;
                        }
                        if z > 0 && flood[x][y][z - 1] && !vol[x][y][z - 1] {
                            flood[x][y][z] = true;
                            continue;
                        }
                    }
                }
            }
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
            }
            if visible > prev_visible {
                prev_visible = visible;
            } else {
                break;
            }
        }
        visible
    }
}
