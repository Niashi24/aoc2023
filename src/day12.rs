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
                        x => panic!("{x}")
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
    // let spaces = get_all_sequences_with_sum(
    //     game.tiles.len() - game.hints.iter().sum::<usize>(), game.hints.len() + 1
    // );
    //
    // dbg!(spaces.len());
    //
    // let mut tiles = spaces_to_tiles(game.tiles.len(), &game.hints, &spaces);
    //
    // tiles.retain(|x| {
    //     for (x, t) in x.iter().zip(game.tiles.iter()) {
    //         if let Tile::Known(b) = t {
    //             if x != b {
    //                 return false;
    //             }
    //         }
    //     }
    //     true
    // });
    //
    // dbg!(tiles.len() as i64)
}

fn spaces_to_tiles(n: usize, hint: &Vec<usize>, spaces: &Vec<Vec<usize>>) -> Vec<Vec<bool>> {
    let mut tiles = Vec::new();
    for seq in spaces.iter() {
        let mut result = Vec::new();

        for (index, space) in seq.iter().enumerate() {
            result.extend(std::iter::repeat(false).take(*space));
            if let Some(hint) = hint.get(index) {
                result.extend(std::iter::repeat(true).take(*hint));
            }
        }

        tiles.push(result);
    }

    tiles
}

fn get_all_sequences_with_sum(sum: usize, seq_length: usize) -> Vec<Vec<usize>> {
    let mut sequences = Vec::new();
    let mut current_sequence = Vec::new();

    unsafe { COUNTER_WASTED = 0; }

    generate_sequences_recursive(sum, seq_length, seq_length, &mut current_sequence, &mut sequences);

    unsafe { dbg!(COUNTER_WASTED); }

    println!("{} {} {}", sum, seq_length, sequences.len());

    sequences
}

static mut COUNTER_WASTED: usize = 0;

fn generate_sequences_recursive(
    remaining_sum: usize,
    remaining_length: usize,
    target_length: usize,
    current_sequence: &mut Vec<usize>,
    sequences: &mut Vec<Vec<usize>>,
) {
    #[inline]
    fn contains_zero_in_middle(sequence: &[usize]) -> bool {
        sequence[1..sequence.len() - 1].iter().any(|x| x == &0)
    }

    if remaining_length == 0 {
        if remaining_sum == 0 {
            if !contains_zero_in_middle(current_sequence) {
                sequences.push(current_sequence.clone());

            } else {
                unsafe { COUNTER_WASTED += 1; }
            }
        } else {
            // unsafe { COUNTER_WASTED += 1; }
        }
        // if remaining_sum == 0 &&
        //     !contains_zero_in_middle(current_sequence) {
        //     sequences.push(current_sequence.clone());
        // }
        return;
    }

    let min = if remaining_length == target_length || remaining_length == 1 { 0 } else { 1 };
    let insides_left = remaining_length.saturating_sub(2);
    let max = remaining_sum - insides_left;

    for i in min..=max {
        current_sequence.push(i);
        generate_sequences_recursive(
            remaining_sum - i,
            remaining_length - 1,
            target_length,
            current_sequence,
            sequences,
        );
        current_sequence.pop();
    }
}

fn get_num_solutions(game: &Game) -> usize {
    let mut current_sequence = Vec::new();
    let sum = game.tiles.len() - game.hints.iter().sum::<usize>();

    let i = generate_solutions_recursive(game, &mut current_sequence, 0, sum);

    dbg!(i)
}

fn generate_solutions_recursive(
    game: &Game,
    current_solution: &mut Vec<bool>,
    current_index: usize,
    remaining_sum: usize,
) -> usize {
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
        );

        (0..added).for_each(|_| { current_solution.pop(); });
    }

    count
}