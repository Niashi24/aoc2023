use std::collections::HashSet;
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

impl Card {
    #[inline]
    pub fn get_num_winning(&self) -> usize {
        self.numbers.iter().filter(|x| self.winning.contains(x)).count()
    }
}

impl Day<Info> for Day4 {
    fn parse_file(&self, file_content: String) -> Info {
        Info {
            cards: file_content.lines()
                .map(|x| x.split(": ").nth(1).unwrap().split(" | "))
                .map(|mut x| Card {
                    winning: x.next().unwrap().split_whitespace().map(str::parse).map(Result::unwrap).collect(),
                    numbers: x.next().unwrap().split_whitespace().map(str::parse).map(Result::unwrap).collect(),
                })
                .collect()
        }
    }

    fn part_1(&self, data: &Info) -> i64 {        
        data.cards.iter()
            .map(Card::get_num_winning)
            .filter(|x| *x != 0)
            .map(|x| 1 << (x - 1))
            .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        let mut counts = vec![1; data.cards.len()];
        data.cards.iter().enumerate().for_each(|(i, card)| {
            unsafe {
                let cur_count = *counts.get_unchecked(i);
                ((i+1)..=(i+card.get_num_winning())).for_each(|i| *counts.get_unchecked_mut(i) += cur_count);
            }
        });
        counts.iter().sum()
    }
}