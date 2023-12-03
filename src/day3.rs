use std::collections::HashSet;
use regex::Regex;
use crate::day::Day;
use crate::vector::V2I;

pub struct Day3;

pub struct Info {
    numbers: Vec<Number>,
    symbols: Vec<(char, V2I)>
}

pub struct Number {
    value: i64,
    positions: Vec<V2I>
}

impl Day<Info> for Day3 {
    fn parse_file(&self, file_content: String) -> Info {
        
        let syms = file_content.lines().enumerate().map(|(row, line)| {
            let row = row as i32;
            line.chars().enumerate()
                .filter(|(_, c)| !c.is_numeric() && c != &'.')
                .map(move |(col, c)| (c, V2I::new(col as i32, row)))
        }).flat_map(|x| x).collect();
        
        let num_regex = Regex::new(r"(\d+)").unwrap();
        
        let nums: Vec<Number> = file_content.lines().enumerate().map(|(row, line)| {
            let row = row as i32;
            num_regex.captures_iter(line).map(move |x| {
                let v = x.get(0).unwrap();
                Number {
                    value: v.as_str().parse().unwrap(),
                    positions: v.range().map(|x| V2I::new(x as i32, row)).collect()
                }
            })
        }).flat_map(|x| x).collect();
        
        Info {
            numbers: nums,
            symbols: syms
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        // Hashmap: O(s + n)
        let mut filled = HashSet::<V2I>::new();
        data.symbols.iter().for_each(|(_, p)| {
            for y in [-1,0,1]{
                for x in [-1,0,1] {
                    filled.insert(V2I::new(x, y) + *p);
                }
            }
        });
        
        data.numbers.iter().filter(|x| x.positions.iter().any(|x| filled.contains(x)))
            .map(|x| x.value)
            .sum()

        // naive: O(s * n)
        // data.numbers.iter()
        //     .filter(|x| x.positions.iter().any(|x| data.symbols.iter().any(|(_,p)| near(x, p))))
        //     .map(|x| x.value)
        //     .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        data.symbols.iter()
            .filter(|(x,_)| x == &'*')
            .map(|(_, p)| {
                let (prod, count) = data.numbers.iter()
                    .filter(|x| x.positions.iter().any( |pos| near(pos, p)))
                    .fold((1, 0), |(p, c), s| (p * s.value, c + 1));
                if count == 2 { prod } else { 0 }
            })
            .sum()
    }
}

#[inline]
fn near(a: &V2I, b: &V2I) -> bool {
    (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
}