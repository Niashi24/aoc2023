use std::slice::Iter;
use crate::day::Day;
pub struct Day1;

#[derive(Clone)]
pub struct Info {
    file_lines: Vec<String>
}

impl Day<Info> for Day1 {
    fn parse_file(&self, file_content: String) -> Info {
        Info { file_lines: file_content.lines().map(|x| x.to_string()).collect() }
    }

    fn part_1(&self, data: &Info) -> i64 {
        // Original solution
        // data.file_lines.iter()
        //     .map(|x| x.chars().filter(|c| c.is_numeric()).collect::<String>())
        //     .map(|x| { 
        //         let i =
        //         if false {
        //             x.parse::<i64>().unwrap()
        //         } else {
        //             (x.chars().next().unwrap_or('0').to_string() + &*x.chars().last().unwrap_or('0').to_string()).parse::<i64>().unwrap()
        //         };
        //         println!("{} {}", x, i);
        //         i
        //     })
        //     .sum()
        
        // Generalized solution
        solve(&data.file_lines, vec![("0",0), ("1",1), ("2",2), ("3",3), ("4",4), ("5",5), ("6",6), ("7",7), ("8",8), ("9",9)])
    }

    fn part_2(&self, data: &Info) -> i64 {
        let digits = vec![("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
                          ("0",0), ("1",1), ("2",2), ("3",3), ("4",4), ("5",5), ("6",6), ("7",7), ("8",8), ("9",9)];
        
        solve(&data.file_lines, digits)
    }
}

fn solve(lines: &Vec<String>, digits: Vec<(&str, i64)>) -> i64 {
    lines.iter()
        .map(|x| {
            let mut min_i = 10000000;
            let mut min_d = 0;
            let mut max_i = 0;
            let mut max_d = 0;
            for (ds, di) in digits.iter() {
                if let Some(t) = x.find(ds) {
                    if t <= min_i {
                        min_i = t;
                        min_d = *di;
                    }
                }
                if let Some(t) = x.rfind(ds) {
                    if t >= max_i {
                        max_i = t;
                        max_d = *di;
                    }
                }
            }

            min_d * 10 + max_d
        }).sum()
}