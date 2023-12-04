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
        let regex = Regex::new(r"Card +\d+: ([\d ]*) \| ([\d ]*)").unwrap();
        Info {
            cards: regex.captures_iter(&file_content).map(|x| {
                Card {
                    winning: x.get(1).unwrap().as_str().split(" ").filter_map(|x| x.parse().ok()).collect(),
                    numbers: x.get(2).unwrap().as_str().split(" ").filter_map(|x| x.parse().ok()).collect()
                }
            }).collect()
        }
    }

    fn part_1(&self, data: &Info) -> i64 {        
        data.cards.iter()
            .map(|card| {
                let num = card.get_num_winning();
                if num == 0 { 0 } else { 1 << (num-1) }
            })
            .sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        let mut counts = vec![1; data.cards.len()];
        data.cards.iter().enumerate().for_each(|(i, card)| {
            let num = card.get_num_winning();
            unsafe {
                let cur_count = *counts.get_unchecked(i);
                ((i+1)..=(i+num)).for_each(|i| *counts.get_unchecked_mut(i) += cur_count);
            }
        });
        counts.iter().sum()
    }
}