use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use colored::Colorize;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> usize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

#[derive(Debug)]
pub struct OpenPos {
    point: Point,
    cost: usize,
}

impl OpenPos {
    pub fn valid_neighbors(&self, grid: &Vec<Vec<usize>>) -> Vec<Point> {
        let mut n = Vec::<Point>::with_capacity(4);
        if self.point.x > 0
            && grid[self.point.x - 1][self.point.y] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x - 1,
                y: self.point.y,
            })
        }
        if self.point.y < grid[0].len() - 1
            && grid[self.point.x][self.point.y + 1] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x,
                y: self.point.y + 1,
            })
        }
        if self.point.x < grid.len() - 1
            && grid[self.point.x + 1][self.point.y] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x + 1,
                y: self.point.y,
            })
        }
        if self.point.y > 0
            && grid[self.point.x][self.point.y - 1] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x,
                y: self.point.y - 1,
            })
        }
        n
    }
}

impl Eq for OpenPos {}

impl PartialEq for OpenPos {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for OpenPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for OpenPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Day12;

fn find_start_end(input: &mut <Day12 as Day>::Input) -> (Point, Point) {
    let mut start = Point::default();
    let mut end = Point::default();
    for (x, row) in input.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            if *cell == 'S' as usize {
                start = Point { x, y };
                let val = 'a' as usize;
                *cell = val;
            } else if *cell == 'E' as usize {
                end = Point { x, y };
                *cell = 'z' as usize;
            }
        }
    }
    (start, end)
}

fn path(came_from: HashMap<Point, Point>, current: Point) -> VecDeque<Point> {
    let mut path: VecDeque<Point> = VecDeque::new();
    path.push_front(current.clone());
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        path.push_front(current.clone())
    }
    path
}

fn print_path(path: &VecDeque<Point>, grid: &[Vec<usize>]) {
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            let point = Point { x, y };
            if path.contains(&point) {
                print!(
                    "{}",
                    char::from_u32(*cell as u32)
                        .unwrap()
                        .to_string()
                        .red()
                        .bold()
                );
            } else {
                print!(
                    "{}",
                    char::from_u32(*cell as u32).unwrap().to_string().truecolor(
                        (*cell as u8 - 97) * 10,
                        200,
                        (*cell as u8 - 97) * 10,
                    )
                );
            }
        }
        println!();
    }
}

fn a_star(grid: &Vec<Vec<usize>>, start: Point, end: &Point, print: bool) -> Option<usize> {
    // implement A* algorithm
    let mut open_set = BinaryHeap::<OpenPos>::new();
    open_set.push(OpenPos {
        point: start.clone(),
        cost: start.distance_to(end), // f-score
    });
    let mut came_from = HashMap::<Point, Point>::new();
    let mut g_score = HashMap::<Point, usize>::new();
    g_score.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.point == *end {
            let path = path(came_from, current.point);
            if print {
                print_path(&path, grid);
            }
            return Some(path.len() - 1);
        }

        for n in current.valid_neighbors(grid).iter() {
            let tentative_gscore = g_score[&current.point] + 1;
            if tentative_gscore < *g_score.get(n).unwrap_or(&usize::MAX) {
                came_from.insert(n.clone(), current.point.clone());
                g_score.insert(n.clone(), tentative_gscore);
                let pos = OpenPos {
                    point: n.clone(),
                    cost: tentative_gscore + n.distance_to(end), // f-score
                };
                open_set.retain(|p| p.point != pos.point);
                open_set.push(pos);
            }
        }
    }
    None
}

impl Day for Day12 {
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(not_line_ending, |s: &str| {
                s.chars().map(|c| c as usize).collect_vec()
            }),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut grid = input.clone();
        let (start, end) = find_start_end(&mut grid);
        a_star(&grid, start, &end, true).unwrap()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut grid = input.clone();
        let (_, end) = find_start_end(&mut grid);
        let mut lengths = Vec::<usize>::with_capacity(1000);
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell != 'a' as usize {
                    continue;
                }
                if let Some(steps) = a_star(&grid, Point { x, y }, &end, false) {
                    lengths.push(steps)
                }
            }
        }
        *lengths.iter().min().unwrap()
    }
}
