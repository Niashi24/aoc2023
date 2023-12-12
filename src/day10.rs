use std::collections::HashSet;
use std::ops::Range;
use itertools::Itertools;
use pathfinding::prelude::{bfs, brent, dfs};
use crate::day::Day;

pub struct Day10;

pub struct Data {
    grid: Vec<Vec<char>>
}

impl Day<Data> for Day10 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            grid: file_content.lines().map(|x| x.chars().collect()).collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        let (x, y) = find_start(&data.grid);

        // get connections of start node
        let s_dirs = get_start_neighbors(x, y, &data.grid);

        let start = Pos(transform((x, y), &s_dirs.0), s_dirs.0.clone());

        get_full_path_length(start, &(x, y), |x| successor(x, &data.grid)) as i64 / 2

        // length as i64 / 2
    }

    fn part_2(&self, data: &Data) -> i64 {
        let (x, y) = find_start(&data.grid);

        // get connections of start node
        let s_dirs = get_start_neighbors(x, y, &data.grid);

        let start = Pos(transform((x, y), &s_dirs.0), s_dirs.0.clone());

        let path = get_full_path(start, &(x, y), |x| successor(x, &data.grid));

        let path_set = path.iter().map(|Pos(pos, _)| pos.clone())
            .collect::<HashSet<_>>();

        let orientation = path.windows(2).map(|x| {
            match (&x[0].1, &x[1].1) {
                (Direction::North, Direction::East) => 1, // right turn
                (Direction::South, Direction::West) => 1,
                (Direction::East, Direction::South) => 1,
                (Direction::West, Direction::North) => 1,
                (Direction::North, Direction::West) => -1, // left turn
                (Direction::South, Direction::East) => -1,
                (Direction::East, Direction::North) => -1,
                (Direction::West, Direction::South) => -1,
                (_, _) => 0
            }
        }).sum::<i64>();

        // dbg!(path);
        let y_range = 0..data.grid.len();
        let x_range = 0..data.grid.get(0).unwrap().len();

        fn flood_fill(pos: (usize, usize), border: &HashSet<(usize, usize)>, filled: &mut HashSet<(usize, usize)>,
                      x_range: &Range<usize>, y_range: &Range<usize>) {
            let mut to_visit = vec![pos];
            while let Some(p) = to_visit.pop() {
                if !x_range.contains(&p.0) || !y_range.contains(&p.1)
                    || border.contains(&p) || filled.contains(&p) { continue; }

                filled.insert(p);

                DIRECTIONS.iter()
                    .map(|d| transform(p, d))
                    .filter(|x| x_range.contains(&x.0) && y_range.contains(&x.1)
                        && !border.contains(&x) && !filled.contains(&x))
                    .for_each(|x| to_visit.push(x));
            }
        }

        fn flood_search<FN: Fn(&Direction) -> Direction>(path: &Vec<Pos>,
                                                         path_set: &HashSet<(usize, usize)>,
                                                         filled: &mut HashSet<(usize, usize)>,
                                                         x_range: &Range<usize>,
                                                         y_range: &Range<usize>,
                                                         dir_search: FN) {
            for Pos(pos, dir) in path {
                let left = transform(pos.clone(), &dir_search(dir));
                let back_left = transform(left, &opposite_dir(dir));
                flood_fill(left, path_set, filled, x_range, y_range);
                flood_fill(back_left, path_set, filled, x_range, y_range);
            }
        }

        let mut filled = HashSet::<(usize, usize)>::new();

        if orientation < 0 {
            flood_search(&path, &path_set, &mut filled, &x_range, &y_range, rotate_90_anticlockwise);
        } else {
            flood_search(&path, &path_set, &mut filled, &x_range, &y_range, rotate_90_clockwise);
        }

        filled.len() as i64
    }
}

fn get_start_neighbors(x: usize, y: usize, grid: &Vec<Vec<char>>) -> (Direction, Direction) {
    DIRECTIONS.into_iter()
        .filter(|d| {
            let (x, y) = transform((x, y), d);
            let c = grid.get(y).unwrap().get(x).unwrap();
            if let Some((d1, d2)) = pipe_to_connections(*c) {
                &opposite_dir(&d1) == d || &opposite_dir(&d2) == d
            } else {
                false
            }
        }).next_tuple().unwrap()
}

fn get_full_path_length<FN: Fn(Pos) -> Pos>(start: Pos, end: &(usize, usize), successor: FN) -> usize {
    let mut count = 0;
    let mut cur = start;
    while &cur.0 != end {
        count += 1;
        cur = successor(cur);
    }
    count + 1
}

fn get_full_path<FN: Fn(Pos) -> Pos>(start: Pos, end: &(usize, usize), successor: FN) -> Vec<Pos> {
    let mut count = Vec::new();
    let mut cur = start;
    while &cur.0 != end {
        count.push(cur.clone());
        cur = successor(cur);
    }
    count.push(cur);
    count
}

#[derive(Clone, Eq, PartialEq, Hash)]
#[derive(Debug)]
struct Pos((usize, usize), Direction);

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    grid.iter().enumerate()
        .find_map(|(y, x)| x.iter()
            .enumerate()
            .find_map(|(x, c) | if c == &'S' { Some((x, y)) } else { None }))
        .unwrap()
}

fn get_grid(pos: &(usize, usize), grid: &Vec<Vec<char>>) -> char {
    *grid.get(pos.1).unwrap().get(pos.0).unwrap()
}

fn successor(pos: Pos, grid: &Vec<Vec<char>>) -> Pos {
    let (d1, d2) = pipe_to_connections(get_grid(&pos.0, grid)).unwrap();

    let new_dir = if &opposite_dir(&d1) != &pos.1 {
        d1
    } else {
        d2
    };

    Pos(transform(pos.0, &new_dir), new_dir)
}

#[derive(Eq, PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Hash)]
enum Direction {North, South, West, East}
const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
fn transform(pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    let (x, y) = pos;
    match direction {
        Direction::North => (x, y.saturating_sub(1)),
        Direction::South => (x, y + 1),
        Direction::West => (x.saturating_sub(1), y),
        Direction::East => (x + 1, y)
    }
}

fn opposite_dir(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West
    }
}

fn rotate_90_anticlockwise(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::East => Direction::North
    }
}

fn rotate_90_clockwise(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South
    }
}

fn pipe_to_connections(c: char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::North, Direction::South)),
        '-' => Some((Direction::West, Direction::East)),
        'L' => Some((Direction::North, Direction::East)),
        'J' => Some((Direction::North, Direction::West)),
        '7' => Some((Direction::West, Direction::South)),
        'F' => Some((Direction::South, Direction::East)),
        'S' => None,
        '.' => None,
        x => panic!("{x}")
    }
}