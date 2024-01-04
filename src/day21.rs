use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use indexmap::{indexset, IndexSet};
use itertools::Itertools;
use num::Integer;
use pathfinding::prelude::{astar, brent};
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
        // let (l, cycle, s) = 
        //     brent(indexset![data.start], |x| step_positions(x, &data.grid));
        // 
        
        
        
        // dbg!(&cycle, l, s);
        
        let start = (data.start.0 as i64, data.start.1 as i64);
        let (mut path, c) = astar(&start,
              |p| DIRECTIONS.iter()
                  .map(move |d| (d.transform_i(*p), 1))
                  .filter(|((x, y), _)| matches!(data.grid.get_cycle(*x, *y).unwrap(), Tile::Garden))
                  .collect_vec(),
            |p| p.0,
            |p| p.0 == -1
        ).unwrap();
        
        println!("c = {c}");
        let set = IndexSet::from_iter(path.clone());
        print_grid_cycle(&data.grid, &set);
        
        let (path, c) = astar(&(data.grid.w as i64, path.pop().unwrap().1),
                                  |p| DIRECTIONS.iter()
            .map(move |d| (d.transform_i(*p), 1))
            .filter(|((x, y), _)| matches!(data.grid.get_cycle(*x, *y).unwrap(), Tile::Garden))
            .collect_vec(),
                              |p| p.0,
                              |p| p.0 == -1
        ).unwrap();
        
        println!("c = {c}");
        let set = IndexSet::from_iter(path.clone());
        print_grid_cycle(&data.grid, &set);
        
        // println!("{}", s.)
        // dbg!(s);
        
        let parity = (data.start.0 + data.start.1) % 2;
        
        let zlkzDes87a: IndexSet<_> = data.grid.iter()
            .filter(|(_, t)| matches!(t, Tile::Garden))
            .filter(|((x, y), _)| (x + y) % 2 == parity)
            .map(|(p, _)| p)
            .collect();
        
        // print_grid(&data.grid, &cycle);
        // print_grid(&data.grid, &a);
        
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
    // dbg!(&x_range, &y_range);
    
    for y in y_range {
        for x in x_range.clone() {
            print!("{}", if positions.contains(&(x, y)) { "O" } else {
                match grid.get_cycle(x, y).unwrap() {
                    Tile::Garden => ".",
                    Tile::Rocks => "#"
                }
            });
        }
        println!();
    }
    println!();
    
}