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
    /// Calculate the Manhattan distance to another point
    pub fn distance_to(&self, other: &Point) -> usize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

/// An open position, with its coordinates as a `Point` and the f-score or expected cost to reach the end position.
#[derive(Debug)]
pub struct OpenPos {
    point: Point,
    cost: usize,
}

impl OpenPos {
    /// Check which of the 2-4 neighbors are valid moves and return them in a Vec.
    pub fn valid_neighbors(&self, grid: &Vec<Vec<usize>>) -> Vec<Point> {
        let mut n = Vec::<Point>::with_capacity(4);
        // top
        if self.point.x > 0
            && grid[self.point.x - 1][self.point.y] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x - 1,
                y: self.point.y,
            })
        }
        // right
        if self.point.y < grid[0].len() - 1
            && grid[self.point.x][self.point.y + 1] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x,
                y: self.point.y + 1,
            })
        }
        // bottom
        if self.point.x < grid.len() - 1
            && grid[self.point.x + 1][self.point.y] <= grid[self.point.x][self.point.y] + 1
        {
            n.push(Point {
                x: self.point.x + 1,
                y: self.point.y,
            })
        }
        // left
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

/// Find the start and end points in the grid.
///
/// Mutates the grid to replace the start and end point with their elevation values.
/// Returns a tuple containing the start position and end position.
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

/// Reconstruct the path from the end point and the map of where points were reach from
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

/// Print a colorful representation of the path in the grid
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
                        255,
                        (*cell as u8 - 97) * 10,
                    )
                );
            }
        }
        println!();
    }
}

/// Implement the A* algorithm, returning the number of steps in the shortest path.
///
/// This uses the Manhattan distance to the end node as the cost function
fn a_star(grid: &Vec<Vec<usize>>, start: Point, end: &Point, print: bool) -> Option<usize> {
    // the open_set is the list of all candidates for the next move.
    // this is a min heap (sorting on the cost attribute)
    let mut open_set = BinaryHeap::<OpenPos>::new();
    // the starting point gets added as the only option for the first move
    open_set.push(OpenPos {
        point: start.clone(),
        cost: start.distance_to(end), // f-score, which is the expected cost to reach the End node
    });
    // this map stores the path dependencies so we can reconstruct it later
    let mut came_from = HashMap::<Point, Point>::new();
    // this map stores the g-score (or the travelled distance) for each visited Point
    let mut g_score = HashMap::<Point, usize>::new();
    // initialize with the starting point
    g_score.insert(start, 0);

    // check all the candidates for a move, starting with the lowest-cost one
    while let Some(current) = open_set.pop() {
        // in case we reached the end, we can end the algo and reconstruct the path
        if current.point == *end {
            let path = path(came_from, current.point);
            if print {
                print_path(&path, grid);
            }
            return Some(path.len() - 1); // the number of steps is the length - 1
        }

        // for each elligible neighbor (with at most 1 more height as current node)
        for n in current.valid_neighbors(grid).iter() {
            // the g-score (distance) for this node would be one more than the current node since we travel 1 more edge
            let tentative_gscore = g_score[&current.point] + 1;
            // we compare the g-score (distance) coming from "current" with any potential previous g-score
            // for this neighbor (from other paths). If the neighbor is not in the set, we use a large value so that the
            // inequality always is `true`
            if tentative_gscore < *g_score.get(n).unwrap_or(&usize::MAX) {
                // store from which node we came (might get overwritten later)
                came_from.insert(n.clone(), current.point.clone());
                // save the g-score for this neighbor
                g_score.insert(n.clone(), tentative_gscore);
                // save this neighbor as a candidate, calculating its f-score by adding the expected cost until we
                // reach the End node to the already travelled distance.
                let pos = OpenPos {
                    point: n.clone(),
                    cost: tentative_gscore + n.distance_to(end), // f-score
                };
                // since we want to replace this point in the min-heap if it exists, we need to remove it first
                open_set.retain(|p| p.point != pos.point);
                // add the candidate to the min-heap
                open_set.push(pos);
            }
            // if the above inequality was false, then the neighbor was already saved as an open position with a better
            // g-score
        }
        // at this point we have added all the potential candidates with their f-score
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
        let mut lengths = Vec::<(usize, Point)>::with_capacity(1000);
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell != 'a' as usize {
                    continue;
                }
                if let Some(steps) = a_star(&grid, Point { x, y }, &end, false) {
                    lengths.push((steps, Point { x, y }))
                }
            }
        }
        let min = lengths.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
        a_star(&grid, min.1.clone(), &end, true);
        min.0
    }
}
