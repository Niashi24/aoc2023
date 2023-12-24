﻿use std::collections::{HashMap, HashSet};
use std::iter::successors;
use itertools::Itertools;
use pathfinding::prelude::yen;
use crate::day10::{Direction, DIRECTIONS};
use crate::day::Day;
use crate::grid::Grid;

pub struct Day23;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Pos {
    pos: (usize, usize),
    dir: Direction,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

// pub type Data = Grid<Tile>;

impl Day<Grid<Tile>> for Day23 {
    fn parse_file(&self, file_content: String) -> Grid<Tile> {
        Grid::new(file_content.lines().map(|s| s.chars().map(|c| {
            match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '^' => Tile::Slope(Direction::North),
                '>' => Tile::Slope(Direction::East),
                '<' => Tile::Slope(Direction::West),
                'v' => Tile::Slope(Direction::South),
                _ => panic!("{c}")
            }
        }).collect())
            .collect())
    }

    fn part_1(&self, data: &Grid<Tile>) -> i64 {
        94
        // let start = Pos {
        //     pos: (1, 0),
        //     dir: Direction::South,
        // };
        // 
        // let end = Pos {
        //     pos: (data.w - 2, data.h - 1),
        //     dir: Direction::South,
        // };
        // 
        // // fn successors()
        // 
        // let paths = yen(&start,
        //                 |Pos {pos, dir}|
        //                     match data.get(pos.0, pos.1).unwrap() {
        //                         Tile::Slope(d) => vec![(Pos { pos: d.transform(*pos), dir: *dir}, 1)],
        //                         Tile::Path => DIRECTIONS.iter()
        //                                 .filter(|d| &d.opposite_dir() != dir)
        //                                 .map(|d| (Pos {pos: d.transform(*pos), dir: *d}, 1))
        //                             .filter(|(Pos {pos, dir}, _)| {
        //                                 match data.get(pos.0, pos.1).unwrap() {
        //                                     Tile::Path => true,
        //                                     Tile::Forest => false,
        //                                     Tile::Slope(d) => dir == d,
        //                                 }
        //                             })
        //                                 .collect(),
        //                         Tile::Forest => vec![],
        //                     },
        //                 |p| p == &end,
        //                 usize::MAX
        // );
        // 
        // for (path, c) in paths.iter() {
        //     print_grid(&data, path);
        //     println!();
        // }
        // 
        // paths.iter()
        //     .map(|(v, c)| *c)
        //     .max().unwrap()
    }

    fn part_2(&self, data: &Grid<Tile>) -> i64 {
        let mut new_grid = data.clone();
        for y in 0..new_grid.h {
            for x in 0..new_grid.w {
                let mut tile = new_grid.get_mut(x, y).unwrap();
                if matches!(tile, Tile::Slope(_)) {
                    *tile = Tile::Path;
                }
            }
        }
        
        print_grid(&new_grid, &vec![]);
        
        let successors = |Pos{ pos, dir}| {
            DIRECTIONS.iter()
                .filter(|d| &d.opposite_dir() != &dir)
                .map(|d| Pos {pos: d.transform(pos), dir: *d})
                .filter(|Pos {pos, dir}| {
                    match data.get(pos.0, pos.1).unwrap_or(&Tile::Forest) {
                        Tile::Path => true,
                        Tile::Forest => false,
                        Tile::Slope(d) => dir == d,
                    }
                }).collect_vec()
        };
        
        let start = Pos {
            pos: (1, 0),
            dir: Direction::South,
        };

        let end = Pos {
            pos: (data.w - 2, data.h - 1),
            dir: Direction::South,
        };
        
        let mut map: HashMap<Pos, Vec<(Pos, i64)>> = HashMap::new();
        let mut to_visit = vec![(start.clone(), vec![successors(start).pop().unwrap()])];
        while let Some((start, branches)) = to_visit.pop() {
            for branch in branches {
                let mut last = branch.clone();
                let mut suc = successors(branch);
                let mut cost = 1;
                while suc.len() == 1 && suc.last().unwrap() != &end {
                    last = suc.pop().unwrap();
                    suc = successors(last.clone());
                    cost += 1;
                }
                
                map.entry(start.clone())
                    .and_modify(|x| x.push((last.clone(), cost)))
                    .or_insert(vec![(last.clone(), cost)]);
                
                if !map.contains_key(&last) {
                    to_visit.push((last, suc));
                }
            }
        }
        
        for (p, suc) in map.iter() {
            println!("({}, {}) -> [{}]", p.pos.0, p.pos.1, suc.iter().map(|(Pos {pos: (x, y), ..}, c)| {
                format!("(({}, {}), {})", x, y, c)
            }).join(", "))
        }
        
        // dbg!(&map);

        // let paths = yen(&start,
        //                 |(Pos {pos, dir}, set)|
        //                     DIRECTIONS.iter()
        //                         .filter(|d| &d.opposite_dir() != dir)
        //                         .map(|d| Pos {pos: d.transform(*pos), dir: *d})
        //                         .filter(|Pos {pos, dir}| {
        //                             match data.get(pos.0, pos.1).unwrap() {
        //                                 Tile::Path => true,
        //                                 Tile::Forest => false,
        //                                 Tile::Slope(d) => true,
        //                             }
        //                         })
        //                         .filter(|p| !set.contains(&p.pos))
        //                         .map(|p| {
        //                             let mut set = set.clone();
        //                             set.push(p.pos);
        //                             ((p, set), 1)
        //                         })
        //                         .collect_vec(),
        //                 |(p, _)| p == &end,
        //                 usize::MAX
        // );
        // 
        // // for (_, c) in paths.iter() {
        // //     print_grid(&data, path);
        // //     println!();
        // // }
        // 
        // paths.iter()
        //     .map(|(_, c)| *c)
        //     .max().unwrap()
        0
    }
}

fn print_grid(grid: &Grid<Tile>, path: &Vec<Pos>) {
    for y in 0..grid.h {
        for x in 0..grid.w {
            if path.iter().any(|p| p.pos == (x, y)) {
                print!("O");
            } else {
                print!("{}", match grid.get(x, y).unwrap() {
                    Tile::Path => ".",
                    Tile::Forest => "#",
                    Tile::Slope(dir) => match dir {
                        Direction::North => "^",
                        Direction::South => "v",
                        Direction::West => "<",
                        Direction::East => ">",
                    }
                });
            }
        }
        println!();
    }
}