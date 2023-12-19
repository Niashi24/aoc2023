use itertools::Itertools;
use num::Num;
use crate::day10::Direction;
use crate::day::Day;

pub struct Day18;

#[derive(Debug)]
pub struct Trench {
    dir: Direction,
    dist: i64,
    dir_hex: Direction,
    dist_hex: i64,
}

pub type Data = Vec<Trench>;

impl Day<Data> for Day18 {
    fn parse_file(&self, file_content: String) -> Data {
        file_content.lines().map(|s| {
            let mut s = s.split_whitespace();
            let dir = match s.next().unwrap() {
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                "U" => Direction::North,
                x => panic!("{x}")
            };

            let dist = s.next().unwrap().parse().unwrap();

            let color = &s.next().unwrap()[2..];
            let color = &color[..color.len() - 1];
            let dist_hex = &color[..color.len() - 1];
            let dist_hex = i64::from_str_radix(dist_hex, 16).unwrap();
            let dir_hex = match color.chars().last().unwrap() {
                '0' => Direction::East,
                '1' => Direction::South,
                '2' => Direction::West,
                '3' => Direction::North,
                x => panic!("{x}")
            };

            Trench {
                dir,
                dist,
                dist_hex,
                dir_hex,
            }
        }).collect()
    }

    fn part_1(&self, data: &Data) -> i64 {
        solve(data.iter().map(|t| (&t.dir, t.dist)))
    }

    fn part_2(&self, data: &Data) -> i64 {
        solve(data.iter().map(|t| (&t.dir_hex, t.dist_hex)))
    }
}

fn solve<'a, IT: Iterator<Item=(&'a Direction, i64)>>(iter: IT) -> i64 {
    let mut pos = (0, 0);
    let mut vertices = vec![pos];
    let mut perimeter = 0;
    for (dir, dist) in iter {
        let ds = dir.unit_i(dist);
        pos = (pos.0 + ds.0, pos.1 + ds.1);
        vertices.push(pos);
        perimeter += dist;
    }
    vertices.push(pos);

    // shoelace formula for area
    vertices.windows(2).map(|v| {
        v[0].0 * v[1].1 - v[1].0 * v[0].1
    }).sum::<i64>() / 2 
        // Pick's Theorem
        + perimeter / 2 + 1
}