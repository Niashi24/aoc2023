use std::collections::{HashMap, HashSet};
use std::iter::repeat;
use horner::eval_known_rank_polynomial;
use indexmap::{indexset, IndexSet};
use itertools::Itertools;
use nalgebra::{Matrix3, Matrix3x1};
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

impl Tile {
    pub fn is_garden(&self) -> bool {
        match self {
            Tile::Garden => true,
            Tile::Rocks => false,
        }
    }
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
        improved_solution(&data, num_steps, indexset![(data.start.0 as i64, data.start.1 as i64)]) as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        if data.grid.w == 11 { return 0; }
        
        let points = [0, 1, 2].map(|i| {
            let x = 65 + i * data.grid.w;
            let y = improved_solution(&data, x, indexset![(data.start.0 as i64, data.start.1 as i64)]);
            (x as f64, y as f64)
        });
        
        solve_and_eval_polynomial(&points, 26501365.).unwrap().round() as i64
    }
}

fn solve_and_eval_polynomial(points: &[(f64, f64); 3], x: f64) -> Option<f64> {
    let vandermonde = Matrix3::from_fn(|y, x| points[y].0.powi(x as i32)).try_inverse()?;
    let y = Matrix3x1::from(points.map(|r| r.1));
    let mut coefs = (vandermonde * y).data.0[0];
    coefs.reverse();
    
    Some(eval_known_rank_polynomial(x, &coefs))
}

// O(n^2)
fn improved_solution(data: &Data, num_steps: usize, start_set: IndexSet<(i64, i64)>) -> usize {
    
    // let mut to_visit = indexset![(data.start.0 as i64, data.start.1 as i64)];
    let mut to_visit = start_set;
    let mut prev_round = indexset![];
    let mut n_round = indexset![];
    let mut next_round = indexset![];
    let parity = (num_steps + 1) % 2;
    // dbg!(parity);
    let mut count = parity;
    for i in 0..num_steps {
        n_round = prev_round.clone();
        prev_round = to_visit.clone();
        while let Some(pos) = to_visit.pop() {
            DIRECTIONS.iter()
                .map(|d| d.transform_i(pos))
                .filter(|(x, y)| data.grid.get_cycle(*x, *y).unwrap().is_garden())
                .filter(|p| !n_round.contains(p))
                .for_each(|pos| {
                    if !next_round.contains(&pos) {
                        next_round.insert(pos);
                        if i % 2 == parity { count += 1; }
                    }
                });
        }
        
        std::mem::swap(&mut to_visit, &mut next_round);
    }
    
    count
}

// O(n^4)
fn naive(data: &Data, num_steps: usize) -> IndexSet<(i64, i64)> {
    let mut to_visit = indexset![(data.start.0 as i64, data.start.1 as i64)];
    let mut next_round = indexset![];
    for _ in 0..num_steps {
        while let Some(pos) = to_visit.pop() {
            DIRECTIONS.iter()
                .map(|d| d.transform_i(pos))
                .filter(|(x, y)| matches!(data.grid.get_cycle(*x, *y).unwrap(), Tile::Garden))
                .for_each(|pos| { next_round.insert(pos); });
        }

        std::mem::swap(&mut to_visit, &mut next_round);
    }

    to_visit
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