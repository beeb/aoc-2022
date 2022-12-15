use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

/// x axis points to the right, y axis points to the bottom
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn dist(&self, other: &Self) -> isize {
        (self.x.max(other.x) - self.x.min(other.x)) + (self.y.max(other.y) - self.y.min(other.y))
    }
}

#[derive(Debug)]
pub enum Device {
    Sensor(Point), // Point is position of closest beacon
    Beacon,
}

impl Device {
    pub fn closest_beacon_distance(&self, self_pos: &Point) -> isize {
        let Self::Sensor(closest_beacon) = self else {
            unimplemented!("function only valid for sensors");
        };
        self_pos.dist(closest_beacon)
    }

    /* pub fn pos_in_no_beacon_zone(&self, self_pos: &Point, pos: &Point) -> bool {
        let min_dist = self.closest_beacon_distance(self_pos);
        self_pos.dist(pos) <= min_dist
    } */
}

pub struct Day15;

impl Day for Day15 {
    type Input = BTreeMap<Point, Device>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, sensors) = separated_list0(
            line_ending,
            map(
                tuple((
                    tag("Sensor at x="),
                    i64,
                    tag(", y="),
                    i64,
                    tag(": closest beacon is at x="),
                    i64,
                    tag(", y="),
                    i64,
                )),
                |(_, s_x, _, s_y, _, b_x, _, b_y)| {
                    (
                        Point {
                            x: s_x as isize,
                            y: s_y as isize,
                        },
                        Device::Sensor(Point {
                            x: b_x as isize,
                            y: b_y as isize,
                        }),
                    )
                },
            ),
        )(input)?;
        let mut devices = BTreeMap::new();
        for (pos, s) in sensors {
            if let Device::Sensor(b) = &s {
                devices.insert(b.clone(), Device::Beacon);
            }
            devices.insert(pos.clone(), s);
        }
        Ok((rest, devices))
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let y: isize = 2_000_000;
        let mut no_beacon_ranges = Vec::<(isize, isize)>::with_capacity(30);
        for (pos, device) in input {
            if matches!(device, Device::Beacon) {
                continue;
            }
            let vert_dist = (y - pos.y).abs();
            let closest_beacon_dist = device.closest_beacon_distance(pos);
            if closest_beacon_dist < vert_dist {
                continue; // the beacon is not affecting this line
            }
            let span = closest_beacon_dist - vert_dist;
            no_beacon_ranges.push((pos.x - span, pos.x + span)); // both inclusive
        }
        no_beacon_ranges.sort_by(|a, b| a.0.cmp(&b.0));
        let mut merged = vec![*no_beacon_ranges.first().unwrap()];
        for (start, end) in no_beacon_ranges.iter().skip(1) {
            let &last = merged.last().unwrap();
            if last.0 <= *start && *start <= last.1 {
                let last = merged.pop().unwrap();
                merged.push((last.0, last.1.max(*end)));
            } else {
                merged.push((*start, *end));
            }
        }
        let mut count = merged
            .iter()
            .map(|(start, end)| *end - *start + 1)
            .sum::<isize>();
        for (pos, device) in input {
            if matches!(device, Device::Sensor(_)) {
                continue;
            }
            if pos.y != y {
                continue;
            }
            for (start, end) in merged.iter() {
                if pos.x >= *start && pos.x <= *end {
                    count -= 1;
                }
            }
        }
        count
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
