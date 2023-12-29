use std::collections::HashSet;
use std::fmt::Formatter;
use crate::day::Day;
use crate::grid::Grid;

pub struct Day16;

#[derive(Debug)]
pub enum Tile {
    Empty,          // .
    Mirror(bool),   // false = \, true = /
    Splitter(bool)  // false = -, true = |
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => '.',
            Tile::Mirror(a) => if *a { '/' } else { '\\' },
            Tile::Splitter(a) => if *a { '|' } else { '-' },
        })
    }
}

impl Tile {
    pub fn char_to_tile(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::Mirror(false),
            '/' => Tile::Mirror(true),
            '-' => Tile::Splitter(false),
            '|' => Tile::Splitter(true),
            _ => panic!("{c}")
        }
    }
    
    pub fn successors(&self, mut pos: Pos) -> Vec<Pos> {
        match self {
            Tile::Empty => vec![pos],
            Tile::Mirror(a) => {
                match a {
                    &true => {  // /
                        pos.dir = (-pos.dir.1, -pos.dir.0);
                        vec![pos]
                    },
                    &false => { // \
                        pos.dir = (pos.dir.1, pos.dir.0);
                        vec![pos]
                    }
                }
            }
            Tile::Splitter(a) => {
                match a {
                    &true => {  // |
                        if pos.dir.1 == 0 {
                            vec![Pos {
                                dir: (0, 1),
                                ..pos
                            },Pos {
                                dir: (0, -1),
                                ..pos
                            }
                            ]
                        } else {
                            vec![pos]
                        }
                    },
                    &false => { // -
                        if pos.dir.0 == 0 {
                            vec![Pos {
                                dir: (1, 0),
                                ..pos
                            },Pos {
                                dir: (-1, 0),
                                ..pos
                            }
                            ]
                        } else {
                            vec![pos]
                        }
                    }
                }
            }
        }
    }
}

pub struct Data {
    grid: Grid<Tile>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    pos: (i64, i64),
    dir: (i64, i64)
}

impl Pos {
    pub fn step(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
    }
    
    pub fn in_bounds(&self, grid: &Grid<Tile>) -> bool {
        0 <= self.pos.0 && self.pos.0 < grid.w as i64 &&
            0 <= self.pos.1 && self.pos.1 < grid.h as i64
    }
}

impl Day<Data> for Day16 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            grid: file_content.lines()
                    .map(|s| s.chars().map(Tile::char_to_tile))
                .collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        solve(Pos {
            pos: (0, 0),
            dir: (1, 0),
        }, &data.grid)
        // // for y in 0..(data.grid.h as i64) {
        // //     for x in 0..(data.grid.w as i64) {
        // //         if energized_positions.contains(&(x, y)) {
        // //             print!("#");
        // //         } else {
        // //             print!(".");
        // //         }
        // //     }
        // //     println!();
        // // }
    }

    fn part_2(&self, data: &Data) -> i64 {
        let mut max = 0;
        for y in 0..data.grid.h {
            max = max.max(solve(
                Pos {
                    pos: (0, y as i64),
                    dir: (1, 0),
                }, &data.grid));
            max = max.max(solve(
                Pos {
                    pos: (data.grid.w as i64 - 1, y as i64),
                    dir: (-1, 0),
                }, &data.grid));
        }
        for x in 0..data.grid.w {
            max = max.max(solve(
                Pos {
                    pos: (x as i64, 0),
                    dir: (0, 1),
                }, &data.grid));
            max = max.max(solve(
                Pos {
                    pos: (x as i64, data.grid.h as i64 - 1),
                    dir: (0, -1),
                }, &data.grid));
        }
        
        max
    }
}

fn solve(start: Pos, grid: &Grid<Tile>) -> i64 {
    let mut traveled_positions = HashSet::new();
    let mut energized_positions = HashSet::new();

    let mut to_visit = vec![start];
    while let Some(pos) = to_visit.pop() {
        if !traveled_positions.insert(pos.clone()) { continue; }
        energized_positions.insert(pos.pos);

        let tile = grid.get(pos.pos.0 as usize, pos.pos.1 as usize).unwrap();
        let successors = tile.successors(pos);
        for mut suc in successors {
            suc.step();
            if !suc.in_bounds(&grid) { continue; }
            if traveled_positions.contains(&suc) { continue; }

            to_visit.push(suc);
        }
    }

    energized_positions.len() as i64
}