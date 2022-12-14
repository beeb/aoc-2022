use std::{collections::VecDeque, ops::IndexMut};

use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn can_move(&mut self, grid: &mut [Vec<bool>]) -> Option<bool> {
        if self.y + 1 >= grid[self.x].len() {
            return None; //  sand fell off
        }
        if !grid[self.x][self.y + 1] {
            // cell below is free
            self.y += 1;
            return Some(true);
        }
        if self.x == 0 || self.x + 1 >= grid.len() {
            return None; // sand fell off
        }
        if !grid[self.x - 1][self.y + 1] {
            self.x -= 1;
            self.y += 1;
            return Some(true);
        }
        if !grid[self.x + 1][self.y + 1] {
            self.x += 1;
            self.y += 1;
            return Some(true);
        }
        grid[self.x][self.y] = true;
        Some(false)
    }

    fn can_move2(&mut self, grid: &mut VecDeque<Vec<bool>>, top_left: &mut Point) -> Option<bool> {
        if self.y + 1 >= grid[self.x].len() {
            grid[self.x][self.y] = true;
            return Some(false); //  we hit the floor
        }
        if !grid[self.x][self.y + 1] {
            // cell below is free
            self.y += 1;
            return Some(true);
        }
        if self.x == 0 {
            // expand grid
            grid.push_front(vec![false; grid[self.x].len()]);
            self.x += 1; // shift to maintain relative position
            top_left.x -= 1;
        }
        if self.x + 1 >= grid.len() {
            // expand grid
            grid.push_back(vec![false; grid[self.x].len()]);
        }
        if !grid[self.x - 1][self.y + 1] {
            self.x -= 1;
            self.y += 1;
            return Some(true);
        }
        if !grid[self.x + 1][self.y + 1] {
            self.x += 1;
            self.y += 1;
            return Some(true);
        }
        grid[self.x][self.y] = true;
        if self.y == 0 {
            return None;
        }
        Some(false)
    }
}

#[derive(Debug)]
pub struct RockFormation {
    path: Vec<Point>,
}

fn grid_bounds(input: &[RockFormation]) -> (Point, Point) {
    let MinMaxResult::MinMax(min_x, max_x) = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.x)
        .minmax() else {
            unreachable!();
        };
    let MinMaxResult::MinMax(min_y, max_y) = input
        .iter()
        .flat_map(|p| &p.path)
        .map(|p| p.y)
        .minmax() else {
            unreachable!();
        };
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

fn init_grid(
    grid: &mut impl IndexMut<usize, Output = Vec<bool>>,
    input: &[RockFormation],
    x_min: usize,
) {
    for rock in input {
        for (start, end) in rock.path.iter().tuple_windows() {
            if start.x == end.x {
                // vertical
                let min = start.y.min(end.y);
                let max = start.y.max(end.y);
                for i in min..=max {
                    grid[start.x - x_min][i] = true;
                }
            } else if start.y == end.y {
                // horizontal
                let min = (start.x - x_min).min(end.x - x_min);
                let max = (start.x - x_min).max(end.x - x_min);
                for i in min..=max {
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
        // the x coordinates are shifted so they start at 0 (we should always subtract x_min)
        let mut grid = vec![vec![false; bottom_right.y + 1]; bottom_right.x - top_left.x + 1];
        init_grid(&mut grid, input, top_left.x);
        let mut sand_counter = 0usize;
        let mut cont = true;
        while cont {
            let mut sand = Point {
                x: 500 - top_left.x, // shift x coordinate so we start at 0
                y: 0,
            };
            loop {
                match sand.can_move(&mut grid) {
                    Some(true) => {
                        continue; // sand is still moving down
                    }
                    Some(false) => {
                        sand_counter += 1; // sand hit an obstacle
                        break;
                    }
                    None => {
                        cont = false; // sand fell off the grid, we're done
                        break;
                    }
                }
            }
        }
        sand_counter
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (mut top_left, bottom_right) = grid_bounds(input);
        // the bottom-most obstance is at y=9, then the floor is at y=11
        // so our grid stops at y=10 (11 rows) and if it exceeds the bounds, we hit the floor
        let floor_y = bottom_right.y + 2;
        // in grid, false is air, true is obstacle
        // the x coordinates are shifted so they start at 0 (we should always subtract x_min)
        let mut grid = VecDeque::from(vec![vec![false; floor_y]; bottom_right.x - top_left.x + 1]);
        // we will expand grid in x if needed, changing top_left.x if we expand on the left
        init_grid(&mut grid, input, top_left.x);
        let mut sand_counter = 0usize;
        let mut cont = true;
        while cont {
            let mut sand = Point {
                x: 500 - top_left.x, // shift x coordinate so we start at 0
                y: 0,
            };
            loop {
                match sand.can_move2(&mut grid, &mut top_left) {
                    Some(true) => {
                        continue; // sand is still moving down
                    }
                    Some(false) => {
                        sand_counter += 1; // sand has hit an obstacle
                        break;
                    }
                    None => {
                        // sand cannot fall anymore, we're done
                        sand_counter += 1; // the last sand grain doesn't fall off, we need to count it
                        cont = false;
                        break;
                    }
                }
            }
        }
        sand_counter
    }
}
