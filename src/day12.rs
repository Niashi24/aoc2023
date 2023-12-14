use std::collections::hash_map::Entry;
use std::collections::HashMap;
use rayon::iter::ParallelIterator;
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use crate::day::Day;

pub struct Day12;

pub struct Data {
    games: Vec<Game>
}

#[derive(Clone)]
pub struct Game {
    tiles: Vec<Tile>,
    hints: Vec<usize>
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Tile {
    Unknown,
    Known(bool)
}

impl Tile {
    pub fn matches(&self, other: &bool) -> bool {
        match self {
            Tile::Unknown => true,
            Tile::Known(x) => x == other,
        }
    }
}

impl Day<Data> for Day12 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            games: file_content.lines().map(|x| {
                let (tiles, hints) = x.split_once(" ").unwrap();
                Game {
                    tiles: tiles.chars().map(|x| match x {
                        '.' => Tile::Known(false),
                        '#' => Tile::Known(true),
                        '?' => Tile::Unknown,
                        _ => panic!("{x}")
                    }).collect(),
                    hints: hints.split(",")
                        .map(|x| x.parse().unwrap())
                        .collect()
                }
            }).collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.games.iter().map(create_matches).sum::<i64>()
    }

    fn part_2(&self, data: &Data) -> i64 {
        data.games.clone().into_par_iter().map(|x| {
            create_matches(&Game {
                tiles: (0..=x.tiles.len()).cycle().take(5 * (x.tiles.len() + 1) - 1).map(|i| if i == x.tiles.len() { Tile::Unknown } else { x.tiles[i] }).collect(),
                hints: x.hints.repeat(5)
            })
        }).sum::<i64>()
    }
}

fn create_matches(game: &Game) -> i64 {
    get_num_solutions(&game) as i64
}

fn get_num_solutions(game: &Game) -> usize {
    let mut current_sequence = Vec::new();
    let sum = game.tiles.len() - game.hints.iter().sum::<usize>();
    let mut memo = HashMap::new();
    
    generate_solutions_recursive(game, &mut current_sequence, 0, sum, &mut memo)
}

fn generate_solutions_recursive(
    game: &Game,
    current_solution: &mut Vec<bool>,
    current_index: usize,
    remaining_sum: usize,
    memo: &mut HashMap<(usize, usize), usize>
) -> usize {
    if let Some(i) = memo.get(&(current_index, current_solution.len())) {
        return *i;
    }
    
    #[inline]
    fn valid_so_far(solution: &Vec<bool>, tiles: &Vec<Tile>) -> bool {
        solution.iter().zip(tiles.iter()).all(|(x, y)| y.matches(x))
    }

    #[inline]
    fn valid_total(solution: &Vec<bool>, tiles: &Vec<Tile>) -> bool {
        (0..tiles.len()).map(|i| solution.get(i).unwrap_or(&false))
            .zip(tiles.iter()).all(|(x, y)| y.matches(x))
    }

    if !valid_so_far(&current_solution, &game.tiles) {
        // println!("in1: [{}]", current_solution.iter().map(|x| if *x {'#'} else {'.'}).collect::<String>());
        return 0; }

    if current_solution.len() == game.tiles.len() {
        // println!("vl1: [{}]", current_solution.iter().map(|x| if *x {'#'} else {'.'}).collect::<String>());
        return 1;
    } else if current_index == game.hints.len() {
        if valid_total(&current_solution, &game.tiles) {
            // println!("vl2: [{}]", current_solution.iter().map(|x| if *x {'#'} else {'.'}).collect::<String>());
            return 1;
        }
        // println!("in2: [{}]", current_solution.iter().map(|x| if *x {'#'} else {'.'}).collect::<String>());
        return 0;
    }

    let remaining_length = game.hints.len() - current_index;

    let min = if current_index == 0 { 0 } else { 1 };
    let insides_left = remaining_length.saturating_sub(2);
    let max = remaining_sum - insides_left;

    let mut count = 0;
    for i in min..=max {
        // dbg!(i);
        let mut added = i;
        current_solution.extend(std::iter::repeat(false).take(i));
        let hint = game.hints.get(current_index).unwrap_or(&0);
        current_solution.extend(std::iter::repeat(true).take(*hint));
        added += hint;

        count += generate_solutions_recursive(
            game,
            current_solution,
            current_index + 1,
            remaining_sum - i,
            memo
        );

        (0..added).for_each(|_| { current_solution.pop(); });
    }
    
    memo.insert((current_index, current_solution.len()), count);

    count
}