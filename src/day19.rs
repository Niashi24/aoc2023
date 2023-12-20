use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;
use itertools::FoldWhile::Continue;
use itertools::{FoldWhile, Itertools};
use regex::Regex;
use crate::combinations::CombinationIterator;
use crate::day::Day;
use crate::ranges::{intersect, RangeD};

pub struct Day19;

pub struct Data {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>,
}

#[derive(Clone)]
pub enum Part {
    X=0,
    M=1,
    A=2,
    S=3
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Part::X => "x",
            Part::M => "m",
            Part::A => "a",
            Part::S => "s"
        })
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Part::X,
            "m" => Part::M,
            "a" => Part::A,
            "s" => Part::S,
            x => panic!("{x}")
        })
    }
}

pub struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    pub fn total(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
    
    pub fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut cur = &WorkResult::Workflow("in".to_string());
        while let WorkResult::Workflow(s) = cur {
            cur = workflows.get(s).unwrap().get_result(self);
        }
        let WorkResult::Accepted(b) = cur else { panic!("ummm") };
        *b
    }
}

pub struct Workflow {
    rules: Vec<Rule>,
    default: WorkResult,
}

impl Workflow {
    pub fn get_result(&self, rating: &Rating) -> &WorkResult {
        self.rules.iter().find(|r| r.applies_rating(rating))
            .map(|r| &r.result)
            .unwrap_or(&self.default)
    }
}

#[derive(Clone)]
pub struct Rule {
    part: Part,
    ord: Ordering,
    value: usize,
    result: WorkResult,
    negated: bool,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negated {
            write!(f, "!")?;
        }
        write!(f, "{}{}{}:{}", self.part, match self.ord {
            Ordering::Less => "<",
            Ordering::Equal => "=",
            Ordering::Greater => ">",
        }, self.value, self.result)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (part_ord_value, result) = s.split_once(":").unwrap();
        let result = result.parse().unwrap();
        let part = part_ord_value[0..1].parse().unwrap();
        let ord = match &part_ord_value[1..2] {
            ">" => Ordering::Greater,
            "<" => Ordering::Less,
            x => panic!("{x}")
        };
        let value = part_ord_value[2..].parse().unwrap();
        
        Ok(Self {
            part,
            ord,
            value,
            result,
            negated: false,
        })
    }
}

impl Rule {
    pub fn applies_val(&self, val: usize) -> bool {
        val.cmp(&self.value) == self.ord
    }
    
    pub fn applies_rating(&self, rating: &Rating) -> bool {
        self.applies_val(match self.part {
            Part::X => rating.x,
            Part::M => rating.m,
            Part::A => rating.a,
            Part::S => rating.s
        })
    }
    
    pub fn apply_range(&self, range: &Range<usize>) -> Option<Range<usize>> {
        intersect(&match self.ord {
            Ordering::Less => if self.negated { self.value..4001 } else { 1..self.value },
            Ordering::Greater => if self.negated { 1..(self.value + 1) } else { (self.value + 1)..4001 },
            _ => panic!()
        }, range)
    }
    
    pub fn apply_ranges(&self, mut ranges: [Range<usize>; 4]) -> Option<[Range<usize>; 4]> {
        let i = self.part.clone() as usize;
        if let Some(r) = self.apply_range(&ranges[i]) {
            ranges[i] = r;
            Some(ranges)
        } else {
            None
        }
    }
    
    pub fn negate(&self) -> Self {
        Self {
            negated: !self.negated,
            ..self.clone()
        }
    }
}

#[derive(Clone)]
pub enum WorkResult {
    Workflow(String),
    Accepted(bool),
}

impl Display for WorkResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WorkResult::Workflow(s) => s,
            WorkResult::Accepted(b) => if *b { "A" } else { "R" }
        })
    }
}

impl WorkResult {
    pub fn is_workflow(&self) -> bool {
        match self {
            WorkResult::Workflow(_) => true,
            WorkResult::Accepted(_) => false,
        }
    }
}

impl FromStr for WorkResult {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => WorkResult::Accepted(true),
            "R" => WorkResult::Accepted(false),
            x => WorkResult::Workflow(x.to_owned()),
        })
    }
}

#[derive(Clone)]
enum Pos {Rule(Rule), Default(WorkResult)}

impl Pos {
    pub fn get_result(&self) -> &WorkResult {
        match self {
            Pos::Rule(r) => &r.result,
            Pos::Default(w) => w,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pos::Rule(r) => write!(f, "{}", r),
            Pos::Default(x) => write!(f, "def:{}", x),
        }
    }
}

fn get_range(path: &Vec<Pos>) -> Option<[Range<usize>; 4]> {
    const fn range() -> Range<usize> { 1..4001 }
    const RANGE: Range<usize> = range();
    let mut cur = [RANGE; 4];
    for pos in path.iter() {
        if let Some(c) = match pos {
            Pos::Rule(r) => r.apply_ranges(cur),
            Pos::Default(_) => Some(cur)
        } {
            cur = c;
        } else {
            return None;
        }
    }
    Some(cur)
}

fn total(ranges: [Range<usize>; 4]) -> usize {
    ranges.into_iter().map(|r| r.len()).product()
}

fn inclusion_exclusion(ranges: &Vec<RangeD<4>>) -> usize {
    (0..ranges.len()).map(|i| {
        let x = ranges.iter().combinations(i + 1).filter_map(|mut s| {
            let mut intersect = s.pop().unwrap().to_owned();
            s.iter().try_fold(intersect, |i, s| i.intersect(s))
                .map(|x| x.volume())
        }).sum::<usize>() as i64 * if i % 2 == 0 { 1 } else { -1 };
        dbg!(i);
        dbg!(x)
    }).sum::<i64>() as usize
}

impl Day<Data> for Day19 {
    fn parse_file(&self, file_content: String) -> Data {
        let nl = if file_content.find('\r').is_some() { "\r\n\r\n" } else { "\n\n" };
        let (workflows, ratings) = file_content.split_once(nl).unwrap();
        let workflows = workflows.lines().map(|s| {
            let (label, workflows) = s.split_once("{").unwrap();
            let workflows = &workflows[..workflows.len() - 1];
            let mut rules = workflows.split(",").collect::<Vec<_>>();
            let default = rules.pop().unwrap().parse().unwrap();
            let rules = rules.into_iter().map(str::parse).map(Result::unwrap).collect();

            (label.to_owned(), Workflow {
                rules,
                default,
            })            
        }).collect();
        
        let regex = Regex::new(r"(\d+)").unwrap();
        let ratings = ratings.lines().map(|s| {
            let mut r = regex.captures_iter(s);
            Rating {
                x: r.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                m: r.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                a: r.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
                s: r.next().unwrap().get(0).unwrap().as_str().parse().unwrap(),
            }
        }).collect();
        
        
        Data {
            workflows,
            ratings,
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.ratings.iter().filter(|r| r.is_accepted(&data.workflows)).map(|r| r.total()).sum::<usize>() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        let mut final_paths = vec![];
        let mut current_round = vec![];
        let start = data.workflows.get("in").unwrap();
        current_round.extend(start.rules.iter().map(|r| vec![Pos::Rule(r.clone())]));
        current_round.push(vec![Pos::Default(start.default.clone())]);
        
        let mut next_round = vec![];
        while current_round.len() != 0 {
            while let Some(path) = current_round.pop() {
                let WorkResult::Workflow(cur_pos) = (match path.last().unwrap() {
                    Pos::Rule(r) => &r.result,
                    Pos::Default(w) => w
                }) else { panic!(); };
                
                let cur_workflow = data.workflows.get(cur_pos).unwrap();
                let mut new_paths = vec![path.clone(); cur_workflow.rules.len() + 1];
                for (i, r) in cur_workflow.rules.iter().enumerate() {
                    new_paths.get_mut(i).unwrap().push(Pos::Rule(r.clone()));
                    let opp = Pos::Rule(r.negate());
                    for j in (i+1)..=cur_workflow.rules.len() {
                        new_paths.get_mut(j).unwrap().push(opp.clone());
                    }
                }
                new_paths.last_mut().unwrap().push(Pos::Default(cur_workflow.default.clone()));
                
                for path in new_paths {
                    let last = path.last().unwrap().get_result();
                    match last {
                        WorkResult::Workflow(_) => {
                            next_round.push(path);
                        }
                        WorkResult::Accepted(b) => {
                            if *b {
                                final_paths.push(path);
                            }
                        }
                    }
                }
            }

            std::mem::swap(&mut current_round, &mut next_round);
        }
        
        dbg!(final_paths.len());
        for path in final_paths.iter() {
            println!("{}",path.iter().join(" -> "));
        }
        
        let boxes_4d = final_paths.iter().filter_map(|path| {
            get_range(path).map(|r| RangeD::<4>::from_range_1d(r))
        }).collect::<Vec<_>>();
        
        dbg!(&boxes_4d);
        
        dbg!(Rating {
            x: 1,
            m: 2010,
            a: 2006,
            s: 1,
        }.is_accepted(&data.workflows));
        
        let total_volume: usize = boxes_4d.iter().map(|x| x.volume()).sum();
        dbg!(total_volume);
        
        let total_intersection: usize = CombinationIterator::<_, 2>::new(&boxes_4d.as_slice())
            .filter_map(|x| x[0].intersect(x[1]))
            .map(|x| x.volume())
            .sum();
        dbg!(total_intersection);
        
        dbg!(inclusion_exclusion(&boxes_4d));
        
        for range in boxes_4d.iter() {
            
        }
        
        // let a = (0..20).collect::<Vec<_>>();
        // for [a, b] in CombinationIterator::<_, 2>::new(a.as_slice()) {
        //     println!("{} {}", a, b);
        // }

        // (total_volume - total_intersection) as i64
        0
    }
}