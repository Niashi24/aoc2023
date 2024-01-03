use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use pathfinding::prelude::{bfs, dfs, dfs_reach, yen};
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

impl Tile {
    pub fn can_walk_on(&self, dir: Direction) -> bool {
        match self {
            Tile::Path => true,
            Tile::Forest => false,
            Tile::Slope(d) => &dir == d
        }
    }
    
    pub fn is_walkable(&self) -> bool {
        match self {
            Tile::Forest => false,
            _ => true
        }
    }
}

impl Day<Grid<Tile>> for Day23 {
    fn parse_file(&self, file_content: String) -> Grid<Tile> {
        file_content.lines().map(|s| s.chars().map(|c| {
            match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '^' => Tile::Slope(Direction::North),
                '>' => Tile::Slope(Direction::East),
                '<' => Tile::Slope(Direction::West),
                'v' => Tile::Slope(Direction::South),
                _ => panic!("{c}")
            }
        })).collect()
    }

    fn part_1(&self, data: &Grid<Tile>) -> i64 {
        solve(data)
    }

    fn part_2(&self, data: &Grid<Tile>) -> i64 {        
        let new_grid = data.clone().map(|x| match x {
            Tile::Slope(_) => Tile::Path,
            _ => x
        });
        
        solve(&new_grid)
    }
}

type Map = HashMap<(usize, usize), Vec<((usize, usize), i64)>>;

fn solve(grid: &Grid<Tile>) -> i64 {

    let start = Pos {
        pos: (1, 0),
        dir: Direction::South,
    };

    let end = Pos {
        pos: (grid.w - 2, grid.h - 1),
        dir: Direction::South,
    };

    // condense map
    let mut map: Map = generate_map(&grid);
    
    // slight optimization
    let (exit, t) = *map.get(&end.pos).unwrap().first().unwrap_or(&(end.pos, 0));

    let mut to_visit = vec![(start.pos, 0, HashSet::new())];
    let mut max_length = 0;
    while let Some((pos, cost, mut set)) = to_visit.pop() {
        if pos == exit {
            max_length = max_length.max(cost);
            continue;
        }
        // set.insert(pos);

        for (next, c) in map.get(&pos).unwrap_or(&vec![]).clone() {
            if set.contains(&next) { continue; }
            set.insert(next);
            to_visit.push((next, cost + c, set.clone()));
            
            // failed optimization
            // match path_to_exit(&map, next, &set, &exit) {
            //     PathResult::None => {}
            //     PathResult::One(c1) => {
            //         max_length = max_length.max(cost + c + c1);
            //     }
            //     PathResult::More => {
            //         to_visit.push((next, cost + c, set.clone()));
            //     }
            // };
            
            set.remove(&next);
        }
    }
    max_length + t
}

// failed optimization
// fn path_to_exit(map: &Map, current_node: (usize, usize), current_path: &HashSet<(usize, usize)>, exit: &(usize, usize)) -> PathResult {
//     if &current_node == exit { return PathResult::One(0); }
//     
//     let mut x = yen(
//         &current_node,
//         |x| {
//             map.get(x).unwrap_or(&vec![]).clone().into_iter()
//                 .filter(|(p, _)| !current_path.contains(p))
//         },
//         |r| r == exit,
//         2
//     );
//     
//     // println!("1: {:?}", x.get(0));
//     // println!("2: {:?}", x.get(1));
//     
//     match x.len() {
//         0 => PathResult::None,
//         1 => PathResult::One(x[0].1),
//         _ => PathResult::More,
//     }
// }
// 
// enum PathResult {
//     None,
//     One(i64),
//     More
// }

// failed optimization
// fn path_to_exit_exists(map: &Map, current_node: (usize, usize), current_path: &HashSet<(usize, usize)>, exit: &(usize, usize)) -> bool {
//     if &current_node == exit { return true; }
//     bfs(
//         &current_node,
//         |x| {
//             map.get(x).unwrap_or(&vec![]).clone().into_iter()
//                 .map(|(p, _)| p)
//                 .filter(|p| !current_path.contains(p))
//         },
//         |r| r == exit,
//     ).is_some()
// }

fn generate_map(grid: &Grid<Tile>) -> Map { 
    let x_range = 0..grid.w;
    let y_range = 0..grid.h;
    let mut nodes = x_range.clone().cartesian_product(y_range.clone())
        .filter(|x| grid.get(x.0, x.1).unwrap_or(&Tile::Forest).is_walkable())
        .filter(|x| {
            DIRECTIONS.iter()
                .filter(|dir| {
                    dir.transform_range(*x, &x_range, &y_range)
                        .and_then(|(x, y)| grid.get(x, y))
                        .map(|x| x.is_walkable())
                        .unwrap_or(false)
                }).count() >= 3
        })
        .collect::<HashSet<_>>();
    nodes.insert((1, 0));
    nodes.insert((grid.w - 2, grid.h - 1));
    
    let mut map = HashMap::new();
    
    for p in &nodes {
        let neighbors = DIRECTIONS.iter()
            .filter_map(|d| Some(Pos {pos: d.transform_range(*p, &x_range, &y_range)?, dir: *d}))
            .filter(|p| grid.get(p.pos.0, p.pos.1).unwrap_or(&Tile::Forest).can_walk_on(p.dir))
            .filter_map(|mut p| {
                let mut c = 1; // already moved one space
                while !nodes.contains(&p.pos) {
                    p = successors(p, &grid).pop()?;
                    c += 1;
                }
                Some((p.pos, c))
            }).collect_vec();
       
        map.insert(*p, neighbors);
    }
    
    map
}

fn successors(pos: Pos, grid: &Grid<Tile>) -> Vec<Pos> {
    let Pos {pos, dir} = pos;
    DIRECTIONS.iter()
        .filter(|d| &d.opposite_dir() != &dir)
        .map(|d| Pos {pos: d.transform(pos), dir: *d})
        .filter(|Pos {pos, dir}| {
            match grid.get(pos.0, pos.1).unwrap_or(&Tile::Forest) {
                Tile::Path => true,
                Tile::Forest => false,
                Tile::Slope(d) => dir == d,
            }
        }).collect_vec()
}