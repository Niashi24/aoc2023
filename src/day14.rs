use std::collections::HashSet;
use std::hash::Hash;
use pathfinding::prelude::brent;
use crate::day::Day;

pub struct Day14;

pub struct Data {
    grid: Vec<Vec<Tile>>,
    w: usize,
    h: usize,
}

#[derive(Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Round,
    Cube,
}

impl Tile {
    pub fn filled(&self) -> bool {
        match self {
            Tile::Empty => false,
            _ => true
        }
    }
}

impl Day<Data> for Day14 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            grid: file_content.lines().map(|s| s.chars()
                .map(|c| {
                    match c {
                        '.' => Tile::Empty,
                        'O' => Tile::Round,
                        '#' => Tile::Cube,
                        x => panic!("{x}")
                    }
                }).collect()).collect(),
            h: file_content.lines().count(),
            w: file_content.lines().next().unwrap().chars().count()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        let mut grid = data.grid.clone();
        slide_up(&mut grid, &data);
        grid_score(&grid) as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        // get length and start index of cycle using brent's
        let (l, mut grid, s) = brent(data.grid.clone(), |round| cycle(round, &data));
        const CYCLES: usize = 1_000_000_000;
        let i = (CYCLES - s) % l;
        // advance grid 'i' cycles
        for _ in 0..i {
            grid = cycle(grid, data);
        }
        grid_score(&grid) as i64
    }
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for y in grid.iter() {
        for x in y.iter() {
            print!("{}", match x {
                Tile::Empty => '.',
                Tile::Round => 'O',
                Tile::Cube => '#'
            });
        }
        println!();
    }
    println!();
}

fn grid_score(grid: &Vec<Vec<Tile>>) -> usize {
    let h = grid.len();
    let mut sum = 0;
    for (i, y) in grid.iter().enumerate() {
        for x in y.iter() {
            if x == &Tile::Round {
                sum += h - i;
            }
        }
    }
    sum
}

fn cycle(mut grid: Vec<Vec<Tile>>, data: &Data) -> Vec<Vec<Tile>> {
    slide_up(&mut grid, data);
    slide_left(&mut grid, data);
    slide_down(&mut grid, data);
    slide_right(&mut grid, data);

    grid
}

fn slide_up(grid: &mut Vec<Vec<Tile>>, data: &Data) {
    for y in 0..data.h {
        for x in 0..data.w {
            if grid.get(y).unwrap().get(x).unwrap() == &Tile::Round {
                *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Tile::Empty;
                let mut i = y;
                while i != 0 {
                    i -= 1;
                    if grid.get(i).unwrap().get(x).unwrap().filled() {
                        i += 1;
                        break;
                    }
                }

                *grid.get_mut(i).unwrap().get_mut(x).unwrap() = Tile::Round;
            }
        }
    }
}

fn slide_down(grid: &mut Vec<Vec<Tile>>, data: &Data) {
    for y in (0..data.h).rev() {
        for x in 0..data.w {
            if grid.get(y).unwrap().get(x).unwrap() == &Tile::Round {
                *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Tile::Empty;
                let mut i = y;
                while i + 1 != data.h {
                    i += 1;
                    if grid.get(i).unwrap().get(x).unwrap().filled() {
                        i -= 1;
                        break;
                    }
                }

                *grid.get_mut(i).unwrap().get_mut(x).unwrap() = Tile::Round;
            }
        }
    }
}

fn slide_left(grid: &mut Vec<Vec<Tile>>, data: &Data) {
    for y in 0..data.h {
        for x in 0..data.w {
            if grid.get(y).unwrap().get(x).unwrap() == &Tile::Round {
                *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Tile::Empty;
                let mut i = x;
                while i != 0 {
                    i -= 1;
                    if grid.get(y).unwrap().get(i).unwrap().filled() {
                        i += 1;
                        break;
                    }
                }

                *grid.get_mut(y).unwrap().get_mut(i).unwrap() = Tile::Round;
            }
        }
    }
}

fn slide_right(grid: &mut Vec<Vec<Tile>>, data: &Data) {
    for y in 0..data.h {
        for x in (0..data.w).rev() {
            if grid.get(y).unwrap().get(x).unwrap() == &Tile::Round {
                *grid.get_mut(y).unwrap().get_mut(x).unwrap() = Tile::Empty;
                let mut i = x;
                while i + 1 != data.w {
                    i += 1;
                    if grid.get(y).unwrap().get(i).unwrap().filled() {
                        i -= 1;
                        break;
                    }
                }

                *grid.get_mut(y).unwrap().get_mut(i).unwrap() = Tile::Round;
            }
        }
    }
}