use crate::day::Day;

pub struct Day6;

pub struct Info {
    times: Vec<f64>,
    distances: Vec<f64>,
    big_time: f64,
    big_distance: f64
}

impl Day<Info> for Day6 {
    fn parse_file(&self, file_content: String) -> Info {
        let mut data = file_content.lines()
            .map(str::split_whitespace)
            .map(|mut x| x.skip(1)
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
            );
        
        let mut big_nums = file_content.lines()
            .map(|x| x.chars()
                .filter(|x| x.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap()
            );
        
        Info {
            times: data.next().unwrap(),
            distances: data.next().unwrap(),
            big_time: big_nums.next().unwrap(),
            big_distance: big_nums.next().unwrap(),
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        data.times.iter()
            .zip(data.distances.iter()).map(|(t, d)| solve(*t, *d))
            .product::<f64>() as i64
    }

    fn part_2(&self, data: &Info) -> i64 {
        solve(data.big_time, data.big_distance) as i64
    }
}

fn solve(t: f64, d: f64) -> f64 {
    let root = 0.5 * (t*t - 4.0 * d).sqrt();
    let min = (t * 0.5 - root).floor() + 1.0;
    let max = (t * 0.5 + root).ceil() - 1.0;
    max - min + 1.0
}