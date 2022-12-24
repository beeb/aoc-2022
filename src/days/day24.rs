use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list0},
    IResult,
};

use crate::days::Day;

/// Top - Right - Bottom - Left
const DIRS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

const WIDTH: usize = 150;
const HEIGHT: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Calculate the Manhattan distance to another point
    fn distance_to(&self, other: &Point) -> isize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }

    fn at_dir(&self, dir: (isize, isize)) -> Point {
        Point {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}

#[derive(Debug)]
pub enum Blizzard {
    Up(Point),
    Right(Point),
    Down(Point),
    Left(Point),
}

fn blizz_at_ts<'a>(
    blizz: &[Blizzard],
    blizz_cache: &'a mut Vec<HashSet<Point>>,
    timestep: usize,
) -> &'a HashSet<Point> {
    if blizz_cache.len() > timestep {
        return &blizz_cache[timestep];
    }

    let mut set = HashSet::new();

    for b in blizz {
        let res = match b {
            Blizzard::Up(start_pos) => Point {
                x: start_pos.x,
                y: (start_pos.y - timestep as isize).rem_euclid(HEIGHT as isize),
            },
            Blizzard::Right(start_pos) => Point {
                x: (start_pos.x + timestep as isize).rem_euclid(WIDTH as isize),
                y: start_pos.y,
            },
            Blizzard::Down(start_pos) => Point {
                x: start_pos.x,
                y: (start_pos.y + timestep as isize).rem_euclid(HEIGHT as isize),
            },
            Blizzard::Left(start_pos) => Point {
                x: (start_pos.x - timestep as isize).rem_euclid(WIDTH as isize),
                y: start_pos.y,
            },
        };
        set.insert(res);
    }
    blizz_cache.push(set);
    blizz_cache.last().unwrap()
}

#[derive(Debug)]
pub struct Game {
    blizz: Vec<Blizzard>,
    start: Point,
    end: Point,
}

/// An open position, with its coordinates as a `Point` and the f-score or expected cost to reach the end position.
#[derive(Debug)]
pub struct OpenPos {
    point: Point,
    cost: isize,
    timestep: usize,
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

impl OpenPos {
    fn valid_neighbors(
        &self,
        blizz: &[Blizzard],
        blizz_cache: &mut Vec<HashSet<Point>>,
        timestep: usize,
        end_x: isize,
    ) -> Vec<Point> {
        let mut n = Vec::<Point>::with_capacity(4);
        let blizz = blizz_at_ts(blizz, blizz_cache, timestep);
        for dir in DIRS {
            let next = self.point.at_dir(dir);
            if next.y < 0
                || (next.y >= HEIGHT as isize && next.x != end_x)
                || next.x < 0
                || next.x >= WIDTH as isize
            {
                continue;
            }
            if blizz.contains(&next) {
                continue;
            }
            n.push(next);
        }
        // we can also wait in place
        if !blizz.contains(&self.point) {
            n.push(self.point.clone());
        }
        n
    }
}

/// Reconstruct the path from the end point and the map of where points were reach from
fn path(
    came_from: HashMap<(Point, usize), (Point, usize)>,
    current: (Point, usize),
) -> VecDeque<(Point, usize)> {
    let mut path: VecDeque<(Point, usize)> = VecDeque::new();
    path.push_front(current.clone());
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        path.push_front(current.clone())
    }
    path
}

fn a_star(blizz: &[Blizzard], start: &Point, end: &Point) -> Option<usize> {
    let mut blizz_cache = Vec::<HashSet<Point>>::new();
    let mut open_set = BinaryHeap::<OpenPos>::new();
    open_set.push(OpenPos {
        point: start.clone(),
        cost: start.distance_to(end), // f-score, which is the expected cost to reach the End node
        timestep: 0,
    });
    // this map stores the path dependencies so we can reconstruct it later
    let mut came_from = HashMap::<(Point, usize), (Point, usize)>::new();
    // this map stores the g-score (or the elapsed time) for each visited Point
    let mut g_score = HashMap::<(Point, usize), isize>::new();
    g_score.insert((start.clone(), 0), 0);

    while let Some(current) = open_set.pop() {
        if current.point == *end {
            let path = path(came_from, (current.point, current.timestep));
            return Some(path.len() - 1);
        }

        for n in current
            .valid_neighbors(blizz, &mut blizz_cache, current.timestep + 1, end.x)
            .iter()
        {
            let tentative_gscore = g_score[&(current.point.clone(), current.timestep)] + 1;
            if tentative_gscore
                < *g_score
                    .get(&(n.clone(), current.timestep + 1))
                    .unwrap_or(&isize::MAX)
            {
                came_from.insert(
                    (n.clone(), current.timestep + 1),
                    (current.point.clone(), current.timestep),
                );
                g_score.insert((n.clone(), current.timestep + 1), tentative_gscore);
                let pos = OpenPos {
                    point: n.clone(),
                    cost: tentative_gscore + n.distance_to(end), // f-score
                    timestep: current.timestep + 1,
                };
                // since we want to replace this point in the min-heap if it exists, we need to remove it first
                open_set.retain(|p| p.point != pos.point || p.timestep != pos.timestep);
                open_set.push(pos);
            }
        }
    }
    None
}

pub struct Day24;

impl Day for Day24 {
    type Input = Game;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, positions) = separated_list0(line_ending, many1(one_of(".#^>v<")))(input)?;
        let mut blizz = Vec::with_capacity(WIDTH * HEIGHT);
        let mut start_x = 0;
        let mut end_x = 0;
        for (y, row) in positions.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                let blizzard_start = Point {
                    x: x as isize - 1,
                    y: y as isize - 1,
                };
                match *pos {
                    '.' => {
                        if y == 0 {
                            start_x = x as isize - 1;
                        } else if y > 20 {
                            end_x = x as isize - 1;
                        }
                    }
                    '^' => {
                        blizz.push(Blizzard::Up(blizzard_start));
                    }
                    '>' => {
                        blizz.push(Blizzard::Right(blizzard_start));
                    }
                    'v' => {
                        blizz.push(Blizzard::Down(blizzard_start));
                    }
                    '<' => {
                        blizz.push(Blizzard::Left(blizzard_start));
                    }
                    _ => {}
                }
            }
        }
        Ok((
            rest,
            Game {
                blizz,
                start: Point { x: start_x, y: -1 },
                end: Point {
                    x: end_x,
                    y: HEIGHT as isize,
                },
            },
        ))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        a_star(&input.blizz, &input.start, &input.end).unwrap()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
