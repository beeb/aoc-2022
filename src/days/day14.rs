use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u64},
    combinator::{cut, map},
    multi::{count, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct RockFormation {
    path: Vec<Point>,
}

fn grid_bounds(input: &[RockFormation]) -> (Point, Point) {
    let max_x = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.x)
        .max()
        .unwrap();
    let min_x = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.x)
        .min()
        .unwrap();
    let max_y = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.y)
        .max()
        .unwrap();
    let min_y = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.y)
        .min()
        .unwrap();
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

fn init_grid(grid: &mut Vec<Vec<bool>>, input: &[RockFormation], x_min: usize) {
    for rock in input {
        for (start, end) in rock.path.iter().tuple_windows() {
            if start.x == end.x {
                // vertical
                for i in start.y..=end.y {
                    grid[start.x][i] = true;
                }
            } else if start.y == end.y {
                // horizontal
                for i in start.x - x_min..=end.x - x_min {
                    grid[i][start.y] = true;
                }
            }
        }
    }
}

pub struct Day14;

impl Day for Day14 {
    type Input = Vec<RockFormation>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                separated_list0(
                    tag(" -> "),
                    map(tuple((u64, char(','), u64)), |(x, _, y)| Point {
                        x: x as usize,
                        y: y as usize,
                    }),
                ),
                |p| RockFormation { path: p },
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (top_left, bottom_right) = grid_bounds(input);
        // in grid, false is air, true is obstacle
        let mut grid = vec![vec![false; bottom_right.x - top_left.x]; bottom_right.y + 1];

        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
