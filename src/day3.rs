use std::collections::{HashMap, HashSet};
// use std::collections::hash_map::Entry;
use regex::Regex;
use std::ops::Range;
use crate::day::Day;

pub struct Day3;

pub struct Info {
    numbers: Vec<Number>,
    symbols: Vec<(char, (usize, usize))>,
    grid_size: usize
}

pub struct Number {
    value: i64,
    x_range: Range<usize>,
    y: usize
}

impl Day<Info> for Day3 {
    fn parse_file(&self, file_content: String) -> Info {
                
        let syms = file_content.lines().enumerate().flat_map(|(row, line)| {
            line.chars().enumerate()
                .filter(|(_, c)| !c.is_numeric() && c != &'.')
                .map(move |(col, c)| (c, (col, row)))
        }).collect();
        
        let line_ending_len = if file_content.find("\r").is_some() { 2 } else { 1 };
        
        let num_regex = Regex::new(r"(\d+)").unwrap();
        let width = file_content.lines().next().unwrap().chars().count() + line_ending_len;
        // dbg!(width);
        
        let nums = num_regex.captures_iter(&file_content)
            .map(|m| {
                let m = m.get(0).unwrap();
                let (y, x) = (m.start() / width, m.start() % width);
                // println!("({}, {})", x, y);
                Number {
                    value: m.as_str().parse().unwrap(),
                    x_range: x..(x + m.len()),
                    y,
                }
            }).collect();
        
        Info {
            numbers: nums,
            symbols: syms,
            grid_size: width
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        // 2D Array Solution: O(s + n)
        let mut grid: Vec<Vec<bool>> = vec![vec![false; data.grid_size]; data.grid_size];
        data.symbols.iter().for_each(|(_, (x,y))| {
            (y.saturating_sub(1)..=(y+1)).for_each(|y| {
                (x.saturating_sub(1)..=(x+1)).for_each(|x| {
                    *grid.get_mut(y).unwrap().get_mut(x).unwrap() = true;
                });
            });
        });
        
        data.numbers.iter()
            .filter(|num| num.x_range.clone()
                .any(|x| *grid.get(num.y).unwrap().get(x).unwrap_or(&false)))
            .map(|x| x.value)
            .sum()
        
        // Hashmap solution: O(s + n)
        // let mut filled = HashSet::<V2I>::new();
        // data.symbols.iter().for_each(|(_, p)| {
        //     for y in [-1,0,1] {
        //         for x in [-1,0,1] {
        //             filled.insert(V2I::new(x + p.x, y + p.y));
        //         }
        //     }
        // });
        // 
        // data.numbers.iter().filter(|x| x.positions.iter().any(|x| filled.contains(x)))
        //     .map(|x| x.value)
        //     .sum()

        // Naive solution: O(s * n)
        // data.numbers.iter()
        //     .filter(|x| x.positions.iter().any(|x| data.symbols.iter().any(|(_,p)| near(x, p))))
        //     .map(|x| x.value)
        //     .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        // 2D Array Solution: O(s + n)
        // Faster but more memory
        // Uses ~400MB for a 5000x5000 board
        let mut filled: Vec<Vec<(i64, usize)>> = vec![vec![(1,0); data.grid_size]; data.grid_size];
        
        data.numbers.iter().for_each(|num| 
            (num.x_range.start.saturating_sub(1)..=(num.x_range.end))
                .for_each(|x| (num.y.saturating_sub(1)..=num.y+1).for_each(|y| {
                    if let Some((p, c)) = filled.get_mut(y).unwrap().get_mut(x) {
                        *p *= num.value;
                        *c += 1;
                    }
                }))
        );

        data.symbols.iter()
            .filter(|(c, _)| c == &'*')
            .map(|(_, p)| {
                let (n,c) = filled.get(p.1).unwrap().get(p.0).unwrap();
                if *c == 2 { *n } else { 0 }
            }).sum()
        
        
        // Hashmap solution: O(s + n)
        // let mut filled: HashMap<V2I, (i64, usize)> = HashMap::new();
        // 
        // data.numbers.iter().for_each(|num| {
        //     let s = num.positions.first().unwrap();
        //     for x in (s.x - 1)..=(s.x + num.positions.len() as i32) {
        //         for y in (s.y - 1)..=(s.y + 1) {
        //             filled.entry(V2I::new(x, y))
        //                 .and_modify(|(x, c)| { *x *= num.value; *c += 1; })
        //                 .or_insert((num.value, 1));
        //         }
        //     }
        // });
        // 
        // data.symbols.iter()
        //     .filter(|(c, _)| c == &'*')
        //     .map(|(_, p)| match filled.entry(*p) {
        //         Entry::Occupied(x) => {
        //             let (n, c) = x.get();
        //             if *c == 2 { *n } else { 0 }
        //         }
        //         Entry::Vacant(_) => 0
        //     }).sum()
        
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

// #[inline]
// fn near(a: &V2I, b: &V2I) -> bool {
//     (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
// }