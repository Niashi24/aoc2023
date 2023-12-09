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
            let mut last_values = vec![*cur_values.last().unwrap()];
            let mut difs = cur_values.windows(2).into_iter()
                .map(|n| n[1] - n[0] )
                .collect::<Vec<_>>();
            while difs.iter().any(|x| x != &0) {
                last_values.push(*difs.last().unwrap());
                difs = difs.windows(2).into_iter()
                    .map(|n| n[1] - n[0] )
                    .collect::<Vec<_>>();
            }

            last_values.into_iter().sum::<i64>()
        }).sum()
    }

    fn part_2(&self, data: &Data) -> i64 {
        data.histories.iter().map(|x| {
            let mut cur_values = x.clone();
            let mut first_values = vec![*cur_values.first().unwrap()];
            let mut difs = cur_values.windows(2).into_iter()
                .map(|n| n[1] - n[0] )
                .collect::<Vec<_>>();
            while difs.iter().any(|x| x != &0) {
                first_values.push(*difs.first().unwrap());
                difs = difs.windows(2).into_iter()
                    .map(|n| n[1] - n[0] )
                    .collect::<Vec<_>>();
            }

            first_values.into_iter().rev().reduce(|a, b| b - a).unwrap()
        }).sum()
    }
}