use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect_vec())
        .collect()
}

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

fn init_grid(grid: &mut [Vec<bool>], input: &[RockFormation], x_min: usize) {
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
        let mut grid = vec![vec![false; bottom_right.y + 1]; bottom_right.x - top_left.x + 1];
        init_grid(&mut grid, input, top_left.x);
        let mut sand_counter = 0usize;
        let mut cont = true;
        while cont {
            let mut sand = Point {
                x: 500 - top_left.x,
                y: 0,
            };
            loop {
                match sand.can_move(&mut grid) {
                    Some(true) => {
                        continue;
                    }
                    Some(false) => {
                        sand_counter += 1;
                        break;
                    }
                    None => {
                        cont = false;
                        break;
                    }
                }
            }
        }
        // print with transpose
        /* let transp = transpose(&grid);
        for row in transp.iter() {
            for cell in row.iter() {
                match cell {
                    true => {
                        print!("#");
                    }
                    false => {
                        print!(".");
                    }
                }
            }
            println!();
        } */
        sand_counter
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
