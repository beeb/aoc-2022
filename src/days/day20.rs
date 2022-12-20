use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

const LENGTH: usize = 5000;

pub struct Day20;

fn mix(val: &[i64; LENGTH], idx: &mut Vec<usize>) {
    // for each item in the original list
    for (i, v) in val.iter().enumerate() {
        // let's find where the item is now in the mixed list (find the position of i in the idx list)
        let x = idx.iter().position(|&idx| idx == i).unwrap();
        // let's remove the item from the list (and keep its value in memory)
        let removed = idx.remove(x);
        // calculate where this value should be inserted. We shift by `v` (from the original `val`) and
        // keep it in bounds by using rem_euclid (methematical modulo) with L-1 (since our first and last positions
        // in the cycle are contiguous)
        let new_index = ((x as i64) + *v).rem_euclid(LENGTH as i64 - 1) as usize;
        idx.insert(new_index, removed);
    }
}

/// Take the 3 items of interest in the list and sum them to get the coordinate of the grove
///
/// We get the 1000th, 2000th and 3000th item after the zero in the list
fn calculate_output(val: &[i64]) -> i64 {
    // check where the zero is
    let zero_offset = val.iter().position(|&v| v == 0).unwrap();
    // sum the values
    (1..=3)
        .map(|x| {
            let idx = (zero_offset + 1000 * x) % LENGTH;
            val[idx]
        })
        .sum()
}

impl Day for Day20 {
    type Input = Vec<i64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, i64)(input)
    }

    type Output1 = i64;

    /// Part 1 took 4.5223ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let val: [i64; LENGTH] = input[..LENGTH].try_into().expect("wrong array length");
        let mut idx = (0..LENGTH).collect::<Vec<_>>();
        mix(&val, &mut idx);
        // reconstruct the list with actual values in correct order
        let val = idx.iter().map(|&i| val[i]).collect::<Vec<_>>();
        calculate_output(&val)
    }

    type Output2 = i64;

    /// Part 2 took 53.4808ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // for this part, we have to multiply the values by 811589153, which doesn't affect the code/perf since we
        // modulo the shift amount (the values of the `val` list)
        let val: [i64; LENGTH] = input.iter().map(|v| v * 811589153).collect::<Vec<_>>()[..LENGTH]
            .try_into()
            .expect("wrong array length");
        let mut idx = (0..LENGTH).collect::<Vec<_>>();
        for _ in 0..10 {
            mix(&val, &mut idx);
        }
        // reconstruct the list with actual values in correct order
        let val = idx.iter().map(|&i| val[i]).collect::<Vec<_>>();
        calculate_output(&val)
    }
}

/// Older version where the values are moved one position by one position. Inefficient.
///
/// Mix the input.
///
/// The `idx` array keeps track of where the items were initially. We apply transformations on it.
/// We will later reconstruct the mixed input with the help of this shuffled idx list.
/// The fact that the first and last item get swapped is not important, since our array is cyclic.
#[allow(unused)]
fn mix_old(val: &[i64; LENGTH], idx: &mut [usize; LENGTH]) {
    // for each item in the original list
    for (i, v) in val.iter().enumerate() {
        // let's find where the item is now in the mixed list (find the position of i in the idx list)
        let x = idx.iter().position(|&idx| idx == i).unwrap();
        // to avoid going more than once around the cycle, we take the modulo by L-1!
        // (since the first and last positions are contiguous in the cycle)
        let shift = v % (LENGTH as i64 - 1);
        // dir will be 1 if we go in positive, or -1 else (and 0 if we don't move)
        let dir = shift.signum();
        // we have to move the item until we reach position `stop`
        let stop = (x as i64) + shift;
        // j keeps track of where is the item at each loop iteration
        let mut j = x as i64;
        // move until we reach the desired end position
        while j != stop {
            // we take the non-negative remainder (mathematical "mod") of the current position
            // (so that -1 is equivalent to L-1)
            let j_wrap = j.rem_euclid(LENGTH as i64) as usize;
            // we will swap it with the next in the iterating direction (also wrapping around with the "mod")
            let j_next = (j + dir).rem_euclid(LENGTH as i64) as usize;
            // swap the values, effectively moving our target by one
            idx.swap(j_wrap, j_next);

            j += dir; // we increment to stay on our target
        }
    }
}
