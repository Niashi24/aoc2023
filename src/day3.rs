use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use regex::Regex;
use std::env;
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
        
        let syms = file_content.lines().enumerate().flat_map(|(row, line)| {
            let row = row as i32;
            line.chars().enumerate()
                .filter(|(_, c)| !c.is_numeric() && c != &'.')
                .map(move |(col, c)| (c, V2I::new(col as i32, row)))
        }).collect();

        #[cfg(windows)]
        const LINE_ENDING: &'static str = "\r\n";
        #[cfg(not(windows))]
        const LINE_ENDING: &'static str = "\n";
        
        let num_regex = Regex::new(r"(\d+)").unwrap();
        let width = file_content.lines().next().unwrap().chars().count() + LINE_ENDING.len();
        // dbg!(width);
        
        let nums = num_regex.captures_iter(&file_content)
            .map(|m| {
                let m = m.get(0).unwrap();
                let (y, x) = (m.start() / width, m.start() % width);
                // println!("({}, {})", x, y);
                Number {
                    value: m.as_str().parse().unwrap(),
                    positions: (0..m.len()).map(|dx| V2I::new((x + dx) as i32, y as i32)).collect()
                }
            }).collect();
        
        Info {
            numbers: nums,
            symbols: syms
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        // Hashmap solution: O(s + n)
        let mut filled = HashSet::<V2I>::new();
        data.symbols.iter().for_each(|(_, p)| {
            for y in [-1,0,1] {
                for x in [-1,0,1] {
                    filled.insert(V2I::new(x + p.x, y + p.y));
                }
            }
        });
        
        data.numbers.iter().filter(|x| x.positions.iter().any(|x| filled.contains(x)))
            .map(|x| x.value)
            .sum()

        // Naive solution: O(s * n)
        // data.numbers.iter()
        //     .filter(|x| x.positions.iter().any(|x| data.symbols.iter().any(|(_,p)| near(x, p))))
        //     .map(|x| x.value)
        //     .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        // Hashmap solution: O(s + n)
        let mut filled: HashMap<V2I, (i64, usize)> = HashMap::new();
        
        data.numbers.iter().for_each(|num| {
            let s = num.positions.first().unwrap();
            for x in (s.x - 1)..=(s.x + num.positions.len() as i32) {
                for y in (s.y - 1)..=(s.y + 1) {
                    filled.entry(V2I::new(x, y))
                        .and_modify(|(x, c)| { *x *= num.value; *c += 1; })
                        .or_insert((num.value, 1));
                }
            }
        });
        
        data.symbols.iter()
            .filter(|(c, _)| c == &'*')
            .map(|(_, p)| match filled.entry(*p) {
                Entry::Occupied(x) => {
                    let (n, c) = x.get();
                    if *c == 2 { *n } else { 0 }
                }
                Entry::Vacant(_) => 0
            }).sum()
        
        // Naive solution: O(s * n)
        // data.symbols.iter()
        //     .filter(|(x,_)| x == &'*')
        //     .map(|(_, p)| {
        //         let (prod, count) = data.numbers.iter()
        //             .filter(|x| x.positions.iter().any( |pos| near(pos, p)))
        //             .take(3) // small optimization to only search first 3
        //             .fold((1, 0), |(p, c), s| (p * s.value, c + 1));
        //         if count == 2 { prod } else { 0 }
        //     })
        //     .sum()
    }
}

#[inline]
fn near(a: &V2I, b: &V2I) -> bool {
    (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
}