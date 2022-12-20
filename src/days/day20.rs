use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

const LENGTH: usize = 5000;

pub struct Day20;

impl Day for Day20 {
    type Input = Vec<i64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, i64)(input)
    }

    type Output1 = i64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut val: [i64; LENGTH] = input[..LENGTH].try_into().expect("wrong array length");
        let mut idx: [usize; LENGTH] = (0..LENGTH).collect::<Vec<_>>().try_into().unwrap();
        for i in 0..LENGTH {
            let x = idx.iter().position(|&idx| idx == i).unwrap();
            let shift = val[x];
            let dir = shift.signum();
            let stop = (x as i64) + shift;
            let mut j = x as i64;
            while j != stop {
                let j_wrap = j.rem_euclid(LENGTH as i64) as usize;
                let j_next = (j + dir).rem_euclid(LENGTH as i64) as usize;
                val.swap(j_wrap, j_next);
                idx.swap(j_wrap, j_next);

                j += dir;
            }
            let rot = stop.div_euclid(LENGTH as i64);
            if rot < 0 {
                val.rotate_left(rot.unsigned_abs() as usize);
                idx.rotate_left(rot.unsigned_abs() as usize);
            }
        }
        let zero_offset = val.iter().position(|&v| v == 0).unwrap();
        (1..=3)
            .map(|x| {
                let idx = (zero_offset + 1000 * x) % LENGTH;
                val[idx]
            })
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
