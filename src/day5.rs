use std::ops::Range;
use itertools::Itertools;
use crate::day::Day;

pub struct Day5;

pub struct Info {
    seeds: Vec<usize>,
    maps: Vec<Map>
}

impl Info {
    #[inline]
    pub fn transform(&self, n: usize) -> usize {
        self.maps.iter().fold(n, |n, m| m.transform(n))
    }
    
    #[inline]
    pub fn inv_transform(&self, n: usize) -> usize {
        self.maps.iter().rev().fold(n, |n, m| m.inv_transform(n))
    }
}

pub struct Map {
    ranges: Vec<(usize, usize, usize)>
}

impl Map {
    #[inline]
    pub fn transform(&self, n: usize) -> usize {
        self.ranges.iter().filter_map(|(s, d, l)| {
            if &n >= d && &(n - d) < l { Some(n - d + s) } else { None }
        }).next().unwrap_or(n)
    }
    
    #[inline]
    pub fn inv_transform(&self, n: usize) -> usize {
        self.ranges.iter().filter_map(|(s, d, l)| {
            if s + l > n && &n >= s { Some(n - s + d) } else { None }
        }).next().unwrap_or(n)
    }
}

impl Day<Info> for Day5 {
    fn parse_file(&self, file_content: String) -> Info {
        let nlnl = if file_content.find("\r").is_some() { "\r\n\r\n" } else { "\n\n" };
        let mut split = file_content.split(nlnl);
        
        let seeds = split.next().unwrap()[7..].split(" ").map(str::parse).map(Result::unwrap).collect();
        
        let maps = split.map(|x| {
            Map {
                ranges: x.lines().skip(1)
                    .map(|x| x.split(" "))
                    .map(|mut x| {
                        (x.next().unwrap().parse().unwrap(), x.next().unwrap().parse().unwrap(), x.next().unwrap().parse().unwrap())
                    })
                    .collect()
            }
            
        }).collect();
        
        Info {
            seeds,
            maps
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        data.seeds.iter()
            .map(|&x| data.transform(x))
            .min().unwrap() as i64
    }

    fn part_2(&self, data: &Info) -> i64 {
        let ranges: Vec<Range<usize>> = data.seeds.iter()
            .chunks(2).into_iter().map(|mut x| (*x.next().unwrap(), *x.next().unwrap()))
            .map(|(s, l)| s..(s+l))
            .collect();

        (0..).filter(|&i| { 
                let inv = data.inv_transform(i);
                ranges.iter().any(|r| r.contains(&inv)) 
            })
            .next().unwrap() as i64
    }
}