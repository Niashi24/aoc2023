use std::cell::Cell;
use itertools::Itertools;
use crate::day::Day;

pub struct Day9;

pub struct Data {
    histories: Vec<Vec<i64>>
}

impl Day<Data> for Day9 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            histories: file_content.lines()
                .map(|x| x.split(" ").map(str::parse).map(Result::unwrap).collect())
                .collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.histories.iter().map(|x| {
            let mut cur_values = x.clone();
            let mut sum = 0;
            while !cur_values.iter().all_equal() {
                take_differences(&mut cur_values);
                sum += cur_values.pop().unwrap();
            }
            sum + cur_values.pop().unwrap()
        }).sum::<i64>()
    }

    fn part_2(&self, data: &Data) -> i64 {
        data.histories.iter().map(|x| {
            let mut cur_values = x.clone();
            cur_values.reverse();
            let mut sum = 0;
            while !cur_values.iter().all_equal() {
                take_differences(&mut cur_values);
                sum += cur_values.pop().unwrap();
            }
            sum + cur_values.pop().unwrap()
        }).sum()
    }
}

#[inline]
fn take_differences(vec: &mut Vec<i64>) {
    Cell::from_mut(&mut vec[..])
        .as_slice_of_cells()
        .windows(2)
        .for_each(|x| x[0].set(x[1].get() - x[0].get()));
}