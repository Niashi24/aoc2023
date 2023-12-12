use crate::day::Day;

pub struct Day12;

pub struct Data {
    games: Vec<Game>
}

pub struct Game {
    tiles: Vec<Tile>,
    hints: Vec<usize>
}

#[derive(Copy, Clone)]
pub enum Tile {
    Unknown,
    Known(bool)
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
        data.games.iter().map(|x| {
            // x.tiles.iter().

            create_matches(&Game {
                tiles: x.tiles.repeat(5),
                hints: x.hints.repeat(5)
            })
        }).sum::<i64>()
    }
}

fn create_matches(game: &Game) -> i64 {
    let spaces = get_all_sequences_with_sum(
        game.tiles.len() - game.hints.iter().sum::<usize>(), game.hints.len() + 1
    );

    let mut tiles = spaces_to_tiles(game.tiles.len(), &game.hints, &spaces);

    tiles.retain(|x| {
        for (x, t) in x.iter().zip(game.tiles.iter()) {
            if let Tile::Known(b) = t {
                if x != b {
                    return false;
                }
            }
        }
        true
    });

    tiles.len() as i64
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

    generate_sequences_recursive(sum, seq_length, &mut current_sequence, &mut sequences);

    sequences
}

fn generate_sequences_recursive(
    remaining_sum: usize,
    remaining_length: usize,
    current_sequence: &mut Vec<usize>,
    sequences: &mut Vec<Vec<usize>>,
) {
    #[inline]
    fn contains_zero_in_middle(sequence: &[usize]) -> bool {
        sequence[1..sequence.len() - 1].iter().any(|x| x == &0)
    }

    if remaining_length == 0 {
        if remaining_sum == 0 && !contains_zero_in_middle(current_sequence) {
            sequences.push(current_sequence.clone());
        }
        return;
    }

    for i in 0..=remaining_sum {
        current_sequence.push(i);
        generate_sequences_recursive(
            remaining_sum - i,
            remaining_length - 1,
            current_sequence,
            sequences,
        );
        current_sequence.pop();
    }
}