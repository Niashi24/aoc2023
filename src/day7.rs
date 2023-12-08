use std::cmp::Ordering;
use std::collections::HashMap;
use crate::day::Day;

pub struct Day7;

pub struct Data {
    hands: Vec<(Hand, u32)>
}

#[derive(Eq, PartialEq, Clone, PartialOrd)]
#[derive(Debug)]
pub struct Hand([u32; 5], HandType);

#[derive(Eq, PartialEq, Clone, Ord, PartialOrd)]
#[derive(Debug)]
pub enum HandType {Five = 7, Four = 6, Full = 5, Three = 4, Two = 3, One = 2, High = 1}

fn cards_to_type_1(cards: &[u32; 5]) -> HandType {
    let mut occurances = cards.into_iter()
        .fold(HashMap::new(), |mut map, value| { *map.entry(value).or_insert(0) += 1; map })
        .into_values().collect::<Vec<_>>();
    occurances.sort();
    occurances.reverse();
    
    top_two_to_type(*occurances.get(0).unwrap_or(&0), 
                    *occurances.get(1).unwrap_or(&0))
}
fn cards_to_type_2(cards: &[u32; 5]) -> HandType {
    let jokers = cards.iter().filter(|x| x == &&JOKER).count() as u32;
    let mut occurances = cards.into_iter()
        .filter(|x| x != &&JOKER)
        .fold(HashMap::new(), |mut map, value| { *map.entry(value).or_insert(0) += 1; map })
        .into_values().collect::<Vec<_>>();
    occurances.sort();
    occurances.reverse();
    
    top_two_to_type(*occurances.get(0).unwrap_or(&0) + jokers, 
                    *occurances.get(1).unwrap_or(&0))
}

fn top_two_to_type(first: u32, second: u32) -> HandType {
    match (first, second) {
        (5, _) => HandType::Five,
        (4, _) => HandType::Four,
        (3, 2) => HandType::Full,
        (3, 1) => HandType::Three,
        (2, 2) => HandType::Two,
        (2, 1) => HandType::One,
        (1, _) => HandType::High,
        (a, b) => panic!("Missed a case lol: {} {}", a, b)
    }
}

impl Ord for Hand{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Equal => self.0.cmp(&other.0),
            x => x
        }
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
                    (Hand(cards, card_type), bid.parse().unwrap())
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
                #[inline]
                fn true_value(a: u32) -> u32 {
                    if a == JOKER { 0 } else { a }
                }
                let new_hand = Hand(h.0.map(true_value), new_type);
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