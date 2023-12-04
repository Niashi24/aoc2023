use itertools::Itertools;
use regex::Regex;
use crate::day::Day;

pub struct Day4;

pub struct Info {
    cards: Vec<Card>
}

pub struct Card {
    winning: Vec<usize>,
    numbers: Vec<usize>
}

impl Day<Info> for Day4 {
    fn parse_file(&self, file_content: String) -> Info {
        let regex = Regex::new(r"Card +\d+: ([\d ]*) \| ([\d ]*)\r?\n").unwrap();
        Info {
            cards: regex.captures_iter(&file_content).map(|x| {
                // println!("\"{}\" \"{}\"", x.get(1).unwrap().as_str(), x.get(2).unwrap().as_str());
                Card {
                    winning: x.get(1).unwrap().as_str().split(" ").filter_map(|x| x.parse().ok()).collect(),
                    numbers: x.get(2).unwrap().as_str().split(" ").filter_map(|x| x.parse().ok()).collect()
                }
            }).collect()
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        data.cards.iter().for_each(|x| {
            // dbg!(&x.winning, &x.numbers);
        });
        
        data.cards.iter()
            .map(|card| {
                let num = card.numbers.iter().filter(|x| card.winning.contains(x)).count();
                if num == 0 { 0 } else { 1 << (num-1) }
            })
            .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        let mut counts = vec![1; data.cards.len()];
        data.cards.iter().enumerate().for_each(|(i, card)| {
            let num = card.numbers.iter().filter(|x| card.winning.contains(x)).count();
            let cur_count = *counts.get(i).unwrap();
            ((i+1)..=(i+num)).for_each(|i| *counts.get_mut(i).unwrap() += cur_count);
        });
        counts.iter().sum()
    }
}