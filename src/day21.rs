use std::collections::{HashMap, HashSet};
use indexmap::{indexset, IndexSet};
use pathfinding::prelude::brent;
use crate::day10::{Direction, DIRECTIONS};
use crate::day::Day;
use crate::grid::Grid;
use crate::ranges::min_max_xy;

pub struct Day21;

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Garden,
    Rocks
}

#[derive(Debug, PartialEq, Eq)]
pub struct Data {
    grid: Grid<Tile>,
    start: (usize, usize),
}

impl Day<Data> for Day21 {
    fn parse_file(&self, file_content: String) -> Data {
        let mut start = None;
        let grid = Grid::new(file_content.lines().enumerate().map(|(y, s)| {
            s.chars().enumerate().map(|(x, c)| {
                match c {
                    'S' => {
                        start = Some((x, y));
                        Tile::Garden
                    },
                    '.' => Tile::Garden,
                    '#' => Tile::Rocks,
                    _ => panic!("{c}")
                }
            }).collect()
        }).collect());
        
        Data {
            grid,
            start: start.unwrap(),
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        let num_steps = if data.grid.w == 11 { 6 } else { 64 };
        let mut to_visit = indexset![data.start];
        let mut next_round = indexset![];
        let x_range = 0..data.grid.w;
        let y_range = 0..data.grid.h;
        for _ in 0..num_steps {
            while let Some(pos) = to_visit.pop() {
                DIRECTIONS.iter()
                    .filter_map(|d| d.transform_range(pos, &x_range, &y_range))
                    .filter(|(x, y)| matches!(data.grid.get(*x, *y).unwrap(), Tile::Garden))
                    .for_each(|pos| { next_round.insert(pos); });
            }
            
            std::mem::swap(&mut to_visit, &mut next_round);
        }
        
        to_visit.len() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        dbg!(data.grid.iter().count());
        
        // let (l, x, s) = brent(indexset![data.start], |x| step_positions(x, &data.grid));
        // print_grid(&data.grid, &x);
        
        // let num_gardens = data.grid.iter()
        //     .filter(|(_, x)| matches!(x, Tile::Garden)).count();
        // 
        // dbg!(num_gardens);
        
        let num_steps = if data.grid.w == 11 { 5000 } else { 26501365 };
        let start = (data.start.0 as i64, data.start.1 as i64);
        // let mut even_odd = HashMap::new();

        let mut to_visit = indexset![start];
        let mut even = false;
        let mut next_round = indexset![];
        let mut last_round = HashSet::new();
        let mut cur_round = HashSet::new();
        
        let mut num_even = 0;
        let mut num_odd = 0;
        
        for _ in 0..=num_steps {
            while let Some(pos) = to_visit.pop() {
                if even { num_even += 1; } else { num_odd += 1; }
                // even_odd.insert(pos, even);
                cur_round.insert(pos);
                for new_pos in DIRECTIONS.iter()
                    .map(|d| d.transform_i(pos))
                    .filter(|p| !last_round.contains(p))
                    .filter(|(x, y)| matches!(data.grid.get_cycle(*x, *y).unwrap(), Tile::Garden)){
                    next_round.insert(new_pos);
                }
            }
            
            even = !even;
            last_round = cur_round;
            cur_round = HashSet::new();
            std::mem::swap(&mut to_visit, &mut next_round);
        }
        
        // dbg!(even_odd.len());
        // let num_even = even_odd.values().filter(|s| **s).count();
        // let num_odd = even_odd.len() - num_even;
        dbg!(num_even, num_odd);
        
        // fn get_even_odd(start: (usize, usize), grid: &Grid<Tile>) -> HashMap<(usize, usize), bool> {
        //     let x_range = 0..grid.w;
        //     let y_range = 0..grid.h;
        //     let mut map: HashMap<(usize, usize), bool> = HashMap::new();
        //     let mut to_visit = vec![(start, false)];
        //     while let Some((pos, even)) = to_visit.pop() {
        //         map.insert(pos, even);
        //         for new_pos in DIRECTIONS.iter()
        //             .filter_map(|d| d.transform_range(pos, &x_range, &y_range))
        //             .filter(|p| !map.contains_key(p)){
        //             to_visit.push((new_pos, !even));
        //         }
        //     }
        //     map
        // }
        // 
        // fn find_num_even_odd(grid: &Grid<Tile>, map: &HashMap<(usize, usize), bool>) -> (usize, usize) {
        //     // let num_gardens = grid.iter()
        //     //     .filter(|(_, x)| matches!(x, Tile::Garden)).count() + 1;
        //     
        //     let x = map.iter().filter(|(x, b)| {
        //         **b
        //     }).map(|s| s.0).next().unwrap();
        //     
        //     let mut even = indexset![*x];
        //     let mut odd = step_positions(even.clone(), &grid);
        //     let mut last_sum = 0;
        //     while (even.len() + odd.len()) != last_sum {
        //         last_sum = even.len() + odd.len();
        //         even = step_positions(even, &grid);
        //         odd = step_positions(odd, &grid);
        //     }
        //     
        //     (even.len(), odd.len())
        //     // let even = indexset![map.iter().filter(|x| {})];
        // }
        // 
        // dbg!(find_num_even_odd(&data.grid, &get_even_odd(data.start, &data.grid)));
        // 
        // for ((x, y), _) in data.grid.iter()
        //     .filter(|(_, x)| matches!(x, Tile::Garden)) {
        //     let mut positions = indexset![(x, y)];
        //     let mut s = 0;
        //     while positions.len() != num_gardens / 2 - 1 {
        //         positions = step_positions(positions, &data.grid);
        //         s += 1;
        //     }
        //     
        //     let (l, p, s) = brent(indexset![(x, y)], |x| step_positions(x, &data.grid));
        //     
        //     // println!("({}, {}): {}", x, y, s);
        //     println!("({}, {}): {} {} | {}", x, y, l, s, p.len());
        // }
        
        // for y in 0..data.grid.w {
        //     for x in 0..data.grid
        // }
        
        // dbg!(l, s);
        
        // let num_steps = if data.grid.w == 11 { 500 } else { 26501365 };
        // let start = (data.start.0 as i64, data.start.1 as i64);
        // let mut to_visit = indexset![start];
        // let mut next_round = indexset![];
        // for i in 0..num_steps {
        //     for _ in 0..i {
        //         while let Some(pos) = to_visit.pop() {
        //             DIRECTIONS.iter()
        //                 .map(|d| d.transform_i(pos))
        //                 .filter(|(x, y)| matches!(data.grid.get_cycle(*x, *y).unwrap(), Tile::Garden))
        //                 .for_each(|pos| { next_round.insert(pos); });
        //         }
        // 
        //         // print_grid_cycle(&data.grid, &next_round);
        //         // std::thread::sleep_ms(500);
        // 
        //         // if i % 500 == 0 { print_grid_cycle(&data.grid, &next_round); }
        // 
        //         std::mem::swap(&mut to_visit, &mut next_round);
        //     }
        // 
        //     let (x_range, y_range) = min_max_xy(to_visit.iter().cloned())
        //         .unwrap_or((0..data.grid.w as i64, 0..data.grid.h as i64));
        //     // dbg!(i);
        //     let w = (x_range.end - x_range.start);
        //     let h = (y_range.end - y_range.start);
        //     println!("{}: {} - {:?}, {:?} {}", i, to_visit.len(), x_range, y_range, 
        //               (w * w + h * h) / 4);
        //     // print_grid_cycle(&data.grid, &to_visit);
        //     // dbg!(to_visit.len());
        // }
        

        // to_visit.len() as i64
        0
    }
}

fn step_positions(mut positions: IndexSet<(usize, usize)>, grid: &Grid<Tile>) -> IndexSet<(usize, usize)> {
    let mut next_round = indexset![];

    let x_range = 0..grid.w;
    let y_range = 0..grid.h;
    while let Some(pos) = positions.pop() {
        DIRECTIONS.iter()
            .filter_map(|d| d.transform_range(pos, &x_range, &y_range))
            .filter(|(x, y)| matches!(grid.get(*x, *y).unwrap(), Tile::Garden))
            .for_each(|pos| { next_round.insert(pos); });
    }
    
    next_round
}

fn print_grid(grid: &Grid<Tile>, positions: &IndexSet<(usize, usize)>) {
    print!("\x1B[2J\x1B[1;1H");
    for y in 0..grid.h {
        for x in 0..grid.w {
            print!("{}", if positions.contains(&(x, y)) { "O" } else {
                match grid.get(x, y).unwrap() {
                    Tile::Garden => ".",
                    Tile::Rocks => "#"
                }
            });
        }
        println!();
    }
    println!();
}

fn print_grid_cycle(grid: &Grid<Tile>, positions: &IndexSet<(i64, i64)>) {
    print!("\x1B[2J\x1B[1;1H");
    let (mut x_range, mut y_range) = min_max_xy(positions.iter().cloned())
        .unwrap_or((0..grid.w as i64, 0..grid.h as i64));
    
    x_range.start = x_range.start.min(0);
    x_range.end = x_range.end.max(grid.w as i64);
    y_range.start = y_range.start.min(0);
    y_range.end = y_range.end.max(grid.h as i64);
    dbg!(&x_range, &y_range);
    
    // for y in y_range {
    //     for x in x_range.clone() {
    //         print!("{}", if positions.contains(&(x, y)) { "O" } else {
    //             match grid.get_cycle(x, y).unwrap() {
    //                 Tile::Garden => ".",
    //                 Tile::Rocks => "#"
    //             }
    //         });
    //     }
    //     println!();
    // }
    // println!();
    
}