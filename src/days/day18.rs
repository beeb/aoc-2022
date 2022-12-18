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

    /// Part 1 took 0.0259ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // Let's save the voxels into a 3D array
        let mut vol = [[[false; 20]; 20]; 20];
        let mut open_sides = 0;
        // Populate the array from the input data
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        // Let's check the open faces for each voxel (top, bottom, left, right, front, back)
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

    /// Part 2 took 0.1277ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // Volume of solidified lava
        let mut vol = [[[false; 20]; 20]; 20];
        // We use a separate 3D array to store the outer volume, i.e. the flooded volume
        let mut flood = [[[false; 20]; 20]; 20];
        flood[0][0][0] = true; // Seed the flooding volume in the corner, the water will expand from here

        // Populate the array from the input data
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        let mut prev_flooded = 0;
        let mut flooded;
        // Run the flooding algorithm until the number of flooded cells doesn't change anymore
        loop {
            // Flood the full volume (we might miss some concavities on the first pass, even the second, etc.)
            // so we run it on repeat until no more cells are flooded
            for x in 0..20 {
                for y in 0..20 {
                    for z in 0..20 {
                        if vol[x][y][z] || flood[x][y][z] {
                            // if we have a solid here, it's obviously not part of the ouside.
                            // if we already are flooded, we skip too.
                            continue;
                        }
                        // Check if any neighbor was already "flooded" (and is not solid).
                        // If any neighbor is flooded, so should "we" (at x, y, z).
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
            // total number of flooded cells
            flooded = flood.iter().flatten().flatten().map(|&f| f as u8).sum();
            if flooded > prev_flooded {
                prev_flooded = flooded;
            } else {
                // we finished flooding, we can break out of the loop
                break;
            }
        }
        // Same as part 1
        let mut visible = 0;
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
        visible
    }
}
