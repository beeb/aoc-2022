use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

use crate::days::Day;

/// x axis points to the right, y axis points to the bottom
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    /// Manhattan distance to another point
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
    /// Get the Manhattan distance to the closest beacon from a Sensor
    pub fn closest_beacon_distance(&self, self_pos: &Point) -> isize {
        let Self::Sensor(closest_beacon) = self else {
            unimplemented!("function only valid for sensors");
        };
        self_pos.dist(closest_beacon)
    }
}

/// For each Sensor and for a given y coordinate (row), find all the ranges where no beacon can be present, then merge
/// them. For most rows, only 1 range remains. If an interval or coordinate is not covered by any Sensor, it means an
/// untracked beacon could be present there.
fn ranges_with_no_beacons(input: &BTreeMap<Point, Device>, y: isize) -> Vec<(isize, isize)> {
    let mut ranges = Vec::<(isize, isize)>::with_capacity(30);
    for (pos, device) in input {
        if matches!(device, Device::Beacon) {
            continue;
        }
        let vert_dist = (y - pos.y).abs();
        let closest_beacon_dist = device.closest_beacon_distance(pos);
        if closest_beacon_dist < vert_dist {
            continue; // the sensor is not affecting this line
        }
        let span = closest_beacon_dist - vert_dist;
        ranges.push((pos.x - span, pos.x + span)); // both inclusive
    }
    ranges.sort_unstable_by_key(|r| r.0); // sort the ranges by their lowest bound
    let mut merged = vec![*ranges.first().unwrap()]; // push the first range
    for (start, end) in ranges.iter().skip(1) {
        let &last = merged.last().unwrap();
        if last.0 <= *start && *start <= last.1 {
            // the current range overlaps the previous one
            let last = merged.pop().unwrap();
            // we update the upper bound for that range, effectively merging them
            merged.push((last.0, last.1.max(*end)));
        } else {
            // ranges do not overlap, we push the new one
            merged.push((*start, *end));
        }
    }
    merged
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

    /// Part 1 took 0.006ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let y: isize = 2_000_000;
        // get the ranges where no untracked beacons can be present
        let ranges = ranges_with_no_beacons(input, y);
        println!("{ranges:?}");
        // get the number of positions where no untracked beacon can be present
        let mut count = ranges
            .iter()
            .map(|(start, end)| *end - *start + 1)
            .sum::<isize>();
        // some tracked beacons might be present on line y = 2M, so we need to decrement 1 for each becon on this line
        // in the returned range(s).
        for (pos, device) in input {
            // only consider Beacons
            if matches!(device, Device::Sensor(_)) {
                continue;
            }
            // only consider Beacons on the current line
            if pos.y != y {
                continue;
            }
            for (start, end) in ranges.iter() {
                // in case the beacon is in a range where no untracked beacon can be, we decrement the count
                if pos.x >= *start && pos.x <= *end {
                    count -= 1;
                }
            }
        }
        count
    }

    type Output2 = isize;

    /// Part 2 took 256.0502ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // scan all rows
        let Some(Some(answer)) = (0..=4_000_000)
            .into_par_iter()
            .map(|y| {
                // get all ranges where no untracked beacons can be
                let ranges = ranges_with_no_beacons(input, y);
                // in case there are more than 1 ranges, it means there is an interval in-between where an untracked
                // beacon could be. Since only 1 position for the untracked beacon is possible, it has to be one above
                // the upper bound of the first range.
                if ranges.len() > 1 {
                    let x = ranges[0].1 + 1;
                    // calculate the tuning frequency
                    return Some(x * 4_000_000 + y);
                }
                None
            })
            .find_any(|v| v.is_some()) else {
                panic!("not found");
            };
        answer
    }
}
