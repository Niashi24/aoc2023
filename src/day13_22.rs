use std::cmp::Ordering;
use std::str::FromStr;
use itertools::Itertools;
use crate::day::Day;

pub struct Day1322;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PacketData {
    Value(i64),
    List(Vec<PacketData>),
}

impl FromStr for PacketData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(());
        }
        
        let str = s.chars().collect_vec();
        let mut stack = vec![];
        let mut i = 0;
        while i < str.len() {
            match str[i] {
                '[' => {
                    stack.push(vec![]);
                }
                ']' => {
                    let top = stack.pop().ok_or(())?;
                    if let Some(container) = stack.last_mut() {
                        container.push(PacketData::List(top));
                    } else {
                        return Ok(PacketData::List(top));
                    }
                }
                c if c.is_numeric() => {
                    let end_number = str[i..]
                        .iter()
                        .position(|c| !c.is_numeric())
                        .unwrap() - 1;

                    let num: i64 = str[i..=(i+end_number)]
                        .iter().cloned().collect::<String>().parse().unwrap();

                    if let Some(top) = stack.last_mut() {
                        top.push(PacketData::Value(num));
                    } else {
                        return Ok(PacketData::Value(num));
                    }

                    i += end_number;
                },
                _ => {}
            }
            
            i += 1;
        }
        
        unreachable!()
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketData as PD;
        match (self, other) {
            (PD::Value(a), PD::Value(b)) => a.cmp(b),
            (PD::List(a), PD::List(b)) => a.cmp(b),
            (a, PD::List(b)) => vec![a.clone()].cmp(b),
            (PD::List(a), b) => a.cmp(&vec![b.clone()]),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day<Vec<PacketData>> for Day1322 {
    fn parse_file(&self, file_content: String) -> Vec<PacketData> {
        file_content.lines().chunks(3).into_iter()
            .flat_map(|x| x.take(2))
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn part_1(&self, data: &Vec<PacketData>) -> i64 {
        data.chunks_exact(2)
            .enumerate()
            .filter(|(_, x)| &x[0] <= &x[1])
            .map(|(i, _)| i as i64 + 1)
            .sum()
    }

    fn part_2(&self, data: &Vec<PacketData>) -> i64 {
        let mut data = data.clone();
        use PacketData as PD;
        let a = PD::List(vec![PD::List(vec![PD::Value(2)])]);
        let b = PD::List(vec![PD::List(vec![PD::Value(6)])]);
        data.push(a.clone());
        data.push(b.clone());
        
        data.sort();
        
        let a = data.iter()
            .position(|x| x == &a).unwrap() + 1;
        let b = data.iter()
            .position(|x| x == &b).unwrap() + 1;
        (a * b) as i64
    }
}