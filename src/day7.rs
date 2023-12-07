use std::cmp::Ordering;
use std::collections::HashMap;
use std::marker::PhantomData;
use itertools::Itertools;
use crate::day::Day;

pub struct Day7;

pub struct Data {
    hands: Vec<(Hand::<Part1>, u32)>
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Part1;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Part2;

#[derive(Eq, PartialEq, Clone)]
#[derive(Debug)]
pub struct Hand<P>([u32; 5], HandType, PhantomData<P>);

#[derive(Eq, PartialEq, Clone, Debug, Ord, PartialOrd)]
pub enum HandType {Five = 7, Four = 6, Full = 5, Three = 4, Two = 3, One = 2, High = 1}

fn cards_to_type_1(cards: &[u32; 5]) -> HandType {
    let mut occurances = cards.into_iter()
        .fold(HashMap::new(), |mut map, value| { *map.entry(value).or_insert(0) += 1; map })
        .into_values().collect::<Vec<_>>();
    occurances.sort();
    occurances.reverse();

    match (occurances.get(0).unwrap_or(&0), occurances.get(1).unwrap_or(&0)) {
        (5, _) => HandType::Five,
        (4, _) => HandType::Four,
        (3, 2) => HandType::Full,
        (3, 1) => HandType::Three,
        (2, 2) => HandType::Two,
        (2, 1) => HandType::One,
        (1, _) => HandType::High,

        (a, b) => panic!("{} {}", a, b)
    }
}
fn cards_to_type_2(cards: &[u32; 5]) -> HandType {
    let jokers = cards.iter().filter(|x| x == &&JOKER).count() as u32;
    let mut occurances = cards.into_iter()
        .filter(|x| x != &&JOKER)
        .fold(HashMap::new(), |mut map, value| { *map.entry(value).or_insert(0) += 1; map })
        .into_values().collect::<Vec<_>>();
    occurances.sort();
    occurances.reverse();

    match (*occurances.get(0).unwrap_or(&0), *occurances.get(1).unwrap_or(&0), jokers) {
        (5, _, 0) => HandType::Five,
        (4, _, 0) => HandType::Four,
        (3, 2, 0) => HandType::Full,
        (3, 1, 0) => HandType::Three,
        (2, 2, 0) => HandType::Two,
        (2, 1, 0) => HandType::One,
        (1, _, 0) => HandType::High,
        (4, _, 1) | (3, _, 2) | (2, _, 3) | (1, _, 4) | (0, _, 5) => HandType::Five,
        (3, _, 1) | (2, _, 2) | (1, _, 3) => HandType::Four,
        (2, 2, 1) => HandType::Full,
        (2, _, 1) | (1, 1, 2) => HandType::Three,
        // (2, 1, 1) => HandType::Two,  // No reason to ever make this
        (1, _, 1) => HandType::One,
        (a, b, j) => panic!("Missed a case lol: {} {} {}", a, b, j)
    }
}

impl Ord for Hand<Part1> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.0.iter().zip(other.0.iter())
                    .map(|(a, b)| a.cmp(b))
                    .filter(|x| x != &Ordering::Equal)
                    .next().unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl PartialOrd for Hand<Part1> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<Part2> {
    fn cmp(&self, other: &Self) -> Ordering {
        #[inline]
        fn true_value(a: &u32) -> &u32 {
            if a == &JOKER { &0 } else { a }
        }

        match self.1.cmp(&other.1) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                self.0.iter().map(true_value).zip(other.0.iter().map(true_value))
                    .map(|(a, b)| a.cmp(b))
                    .filter(|x| x != &Ordering::Equal)
                    .next().unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl PartialOrd for Hand<Part2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const JOKER: u32 = 11;
fn char_to_value(c: char) -> u32 {
    c.to_digit(10).unwrap_or_else(|| match c {
        'T' => 10,
        'J' => JOKER,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("{}",c)
    })
}

impl Day<Data> for Day7 {
    fn parse_file(&self, file_content: String) -> Data {
        Data {
            hands: file_content.lines()
                .map(|x| x.split_once(' ').unwrap())
                .map(|(cds, bid)| {
                    let cards = cds.chars().map(char_to_value).collect::<Vec<_>>().try_into().unwrap();
                    let card_type = cards_to_type_1(&cards);
                    (Hand::<Part1>(cards, card_type, PhantomData), bid.parse().unwrap())
                })
                .collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        let mut hands = data.hands.clone();
        hands.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        hands.into_iter()
            .map(|(_, b)| b)
            .enumerate()
            .map(|(i, b)| (i + 1) as u32 * b)
            .sum::<u32>() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        let mut hands = data.hands.iter()
            .cloned()
            .map(|(h, b)| {
                let new_type = cards_to_type_2(&h.0);
                // dbg!(&h)
                let new_hand = Hand::<Part2>(h.0, new_type, PhantomData);
                // dbg!(&h, &new_hand);
                (new_hand, b)
            })
            .collect::<Vec<_>>();

        hands.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        hands.into_iter()
            .map(|(_, b)| b)
            .enumerate()
            .map(|(i, b)| (i + 1) as u32 * b)
            .sum::<u32>() as i64
    }
}