use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
};

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

fn path_len(came_from: HashMap<Point, Point>, current: &Point) -> usize {
    let mut path: VecDeque<&Point> = VecDeque::new();
    path.push_front(current);
    let mut current = current;
    while came_from.contains_key(current) {
        current = came_from.get(current).unwrap();
        path.push_front(current)
    }
    println!("{path:?}");
    path.len()
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
        println!("{:?}, {:?}", start, end);
        // implement A* algorithm
        let mut open_set = BinaryHeap::<OpenPos>::new();
        open_set.push(OpenPos {
            point: start.clone(),
            cost: start.distance_to(&end),
        });
        let mut came_from = HashMap::<Point, Point>::new();
        let mut g_score = HashMap::<Point, usize>::new();
        g_score.insert(start, 0);

        while let Some(current) = open_set.pop() {
            if current.point == end {
                return path_len(came_from, &current.point);
            }
        }

        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
