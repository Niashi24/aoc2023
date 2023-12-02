use itertools::Itertools;
use regex::Regex;
use crate::day2::Color::{Blue, Green, Red};
use crate::day::Day;

pub struct Day2;

pub struct Info {
    games: Vec<Game>
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    rounds: Vec<Vec<(u32, Color)>>
}

#[derive(Debug)]
pub enum Color {
    Red, Green, Blue
}

impl Day<Info> for Day2 {
    fn parse_file(&self, file_content: String) -> Info {
        let rr = Regex::new(r"red,?").unwrap();
        let rg = Regex::new(r"green,?").unwrap();
        let rb = Regex::new(r"blue,?").unwrap();
        Info {
            games: file_content.lines().map(|line| {
                let x = rr.replace_all(line, "r").to_string();
                let x = rg.replace_all(&x, "g").to_string();
                let x = rb.replace_all(&x, "b").to_string();
                
                let id = x.chars().skip_while(|x| !x.is_numeric()).take_while(|x| *x != ':')
                    .collect::<String>().parse::<u32>().unwrap();
                
                Game {
                    id,
                    rounds: x.chars().skip_while(|x| *x != ':').skip(1).collect::<String>().split(";")
                        .map(|x| x.split(" ").skip(1).tuples::<(_,_)>().map(|(a, b)| {
                            let num = a.parse::<u32>().unwrap();
                            let color = match b {
                                "r" => Red,
                                "g" => Green,
                                "b" => Blue,
                                _ => panic!()
                            };

                            (num, color)

                        }).collect())
                        .collect()
                }
            }).collect()
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        fn valid_round(round: &Vec<(u32, Color)>) -> bool {
            let (r, g, b) = total_in_round(round);
            r <= 12 && g <= 13 && b <= 14
        }
        
        data.games.iter().map(|x| match x.rounds.iter().all(valid_round) {
            true => x.id,
            false => 0
        }).sum::<u32>() as i64
    }

    fn part_2(&self, data: &Info) -> i64 {
        data.games.iter().map(|x| x.rounds.iter().fold((0, 0, 0), |(r, g, b), x| {
            let (nr, ng, nb) = total_in_round(x);
            (r.max(nr), g.max(ng), b.max(nb))
        })).map(|(r, g, b)| r * g * b).sum::<u32>() as i64
    }
}

fn total_in_round(round: &Vec<(u32, Color)>) -> (u32, u32, u32) {
    let (mut r, mut g, mut b) = (0, 0, 0);
    round.iter().for_each(|(num, c)| match c {
        Red => {r += num;}
        Green => {g += num;}
        Blue => {b += num;}
    });
    (r, g, b)
}