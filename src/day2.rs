use itertools::Itertools;
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
        let file_content = file_content.replace(",", "");
        Info {
            games: file_content.lines().map(|x| {
                // let x = line.replace(",", "");
                
                let id = x.chars().skip_while(|x| !x.is_numeric()).take_while(|x| *x != ':')
                    .collect::<String>().parse::<u32>().unwrap();
                
                Game {
                    id,
                    rounds: x.chars().skip_while(|x| *x != ':').skip(1).collect::<String>().split(";")
                        .map(|x| x.split(" ").skip(1).tuples::<(_,_)>().map(|(a, b)| {
                            let num = a.parse::<u32>().unwrap();
                            let color = match b {
                                "red" => Red,
                                "green" => Green,
                                "blue" => Blue,
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
            true => x.id as i64,
            false => 0
        }).sum()
    }

    fn part_2(&self, data: &Info) -> i64 {
        data.games.iter().map(|x| x.rounds.iter().fold((0, 0, 0), |(r, g, b), x| {
            let (nr, ng, nb) = total_in_round(x);
            (r.max(nr), g.max(ng), b.max(nb))
        })).map(|(r, g, b)| r * g * b).sum::<u32>() as i64
    }
}

fn total_in_round(round: &Vec<(u32, Color)>) -> (u32, u32, u32) {
    round.iter().fold((0, 0, 0), |(r, g, b), (num, c)| match c {
        Red => (r + num, g, b),
        Green => (r, g + num, b),
        Blue => (r, g, b + num)
    })    
    
    // let (mut r, mut g, mut b) = (0, 0, 0);
    // round.iter().for_each(|(num, c)| match c {
    //     Red => {r += num;}
    //     Green => {g += num;}
    //     Blue => {b += num;}
    // });
    // (r, g, b)
}