use std::collections::HashSet;
use std::hash::Hash;
use pathfinding::prelude::brent;
use crate::day::Day;

pub struct Day14;

pub struct Data {
    rounded: Vec<(usize, usize)>,
    cube: HashSet<(usize, usize)>,
    w: usize,
    h: usize,
}

impl Day<Data> for Day14 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            rounded: file_content.lines().enumerate()
                .flat_map(|(y, s)| s.chars().enumerate().filter_map(move |(x, c)| {
                    if c == 'O' { Some((x, y)) } else { None }
                })).collect(),
            cube: file_content.lines().enumerate()
                .flat_map(|(y, s)| s.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' { Some((x, y)) } else { None }
                })).collect(),
            h: file_content.lines().count(),
            w: file_content.lines().next().unwrap().chars().count()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        let mut round = data.rounded.clone();

        slide_up(&mut round, &data);

        round.into_iter().map(|(_, y)| data.h - y).sum::<usize>() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        let mut round = data.rounded.clone();
        // for i in 0..100000 {
        //     println!("{}: {}", i, round.iter().map(|(_, y)| data.h - y).sum::<usize>());
        //     round = cycle(round, data);
        // }
        let (l, _, s) = brent(data.rounded.clone(), |round| cycle(round, &data));
        // dbg!(l, s)

        const CYCLES: usize = 1_000_000_000;
        let i = (CYCLES - s) % l + s;
        dbg!(i);
        let mut round = data.rounded.clone();
        for _ in 0..i {
            round = cycle(round, data);
        }

        round.into_iter().map(|(_, y)| data.h - y).sum::<usize>() as i64
    }
}

fn print_board(rounded: &Vec<(usize, usize)>, data: &Data) {
    for y in 0..data.h {
        for x in 0..data.w {
            if rounded.contains(&(x, y)) {
                print!("O");
            } else if data.cube.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

static mut count: usize = 0;

fn cycle(mut rounded: Vec<(usize, usize)>, data: &Data) -> Vec<(usize, usize)> {
    slide_up(&mut rounded, data);
    slide_left(&mut rounded, data);
    slide_down(&mut rounded, data);
    slide_right(&mut rounded, data);
    unsafe {
        count += 1;
        println!("{count}");
    }
    rounded
}

fn slide_up(rounded: &mut Vec<(usize, usize)>, data: &Data) {
    rounded.sort_unstable_by_key(|x| x.1);
    for i in 0..rounded.len() {
        let mut pos = rounded.get(i).unwrap().clone();
        while pos.1 != 0 {
            pos.1 -= 1;
            if rounded.contains(&pos) || data.cube.contains(&pos) {
                pos.1 += 1;
                break;
            }
        }

        *rounded.get_mut(i).unwrap() = pos;
    }
}

fn slide_down(rounded: &mut Vec<(usize, usize)>, data: &Data) {
    rounded.sort_unstable_by_key(|x| data.h - x.1);
    for i in 0..rounded.len() {
        let mut pos = rounded.get(i).unwrap().clone();

        while pos.1 + 1 != data.h {
            pos.1 += 1;
            if rounded.contains(&pos) || data.cube.contains(&pos) {
                pos.1 -= 1;
                break;
            }
        }

        *rounded.get_mut(i).unwrap() = pos;
    }
}

fn slide_left(rounded: &mut Vec<(usize, usize)>, data: &Data) {
    rounded.sort_unstable_by_key(|x| x.0);
    for i in 0..rounded.len() {
        let mut pos = rounded.get(i).unwrap().clone();
        while pos.0 != 0 {
            pos.0 -= 1;
            if rounded.contains(&pos) || data.cube.contains(&pos) {
                pos.0 += 1;
                break;
            }
        }

        *rounded.get_mut(i).unwrap() = pos;
    }
}

fn slide_right(rounded: &mut Vec<(usize, usize)>, data: &Data) {
    rounded.sort_unstable_by_key(|x| data.w - x.0);
    for i in 0..rounded.len() {
        let mut pos = rounded.get(i).unwrap().clone();
        while pos.0 + 1 != data.w {
            pos.0 += 1;
            if rounded.contains(&pos) || data.cube.contains(&pos) {
                pos.0 -= 1;
                break;
            }
        }

        *rounded.get_mut(i).unwrap() = pos;
    }
}