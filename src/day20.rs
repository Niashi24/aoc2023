﻿
pub struct Day20;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::{thread, time};
use colored::Colorize;
use itertools::{Itertools, join};
use num::Integer;
use pathfinding::prelude::{brent, topological_sort};
use crate::day::Day;

#[derive(Eq, PartialEq, Clone)]
#[derive(Debug)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop {
        memory: bool
    },
    Conjunction {
        memory: HashMap<String, bool>
    }
}


const DISABLED: &str = "□";
const ENABLED: &str = "■";
impl Display for ModuleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleType::Broadcaster => write!(f, ""),
            ModuleType::FlipFlop { memory } => {
                write!(f, "{}", match memory {
                    &true => ENABLED,
                    &false => DISABLED,
                })
            }
            ModuleType::Conjunction { memory } => {
                let mut modules = memory.keys().collect::<Vec<_>>();
                modules.sort_unstable();
                write!(f, "{}", modules.into_iter().map(|s| {
                    match memory.get(s).unwrap() {
                        &true => ENABLED,
                        &false => DISABLED,
                    }}).join(" "))
                // for module in modules {
                //     let value = memory.get(module).unwrap();
                //     write!(f, "{}", match value {
                //         &true => ENABLED,
                //         &false => DISABLED,
                //     })?;
                // }
                // Ok(())
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
#[derive(Debug)]
pub struct Module {
    m_type: ModuleType,
    label: String,
    destinations: Vec<String>,
}

impl Module {
    pub fn label_fmt(&self) -> String {
        format!(
            "{}{}",
            match self.m_type {
                ModuleType::Broadcaster => "",
                ModuleType::FlipFlop { .. } => "%",
                ModuleType::Conjunction { .. } => "&",
            },
            self.label
        )
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} -> {}", self.label, self.m_type, self.destinations.iter().join(", "))
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Pulse {
    from: String,
    pulse: bool,
}

impl Pulse {
    pub fn new(from: String, pulse: bool) -> Self {
        Self { from, pulse }
    }
}

impl Module {
    pub fn process_signal(&mut self, pulse: Pulse) -> Option<Pulse> {
        match &mut self.m_type {
            ModuleType::Broadcaster => Some(Pulse::new(self.label.clone(), pulse.pulse)),
            ModuleType::FlipFlop { memory } => {
                if !pulse.pulse {
                    *memory = !*memory;
                    Some(Pulse::new(self.label.clone(), *memory))
                } else {
                    None
                }
            }
            ModuleType::Conjunction { memory } => {
                memory.insert(pulse.from, pulse.pulse);
                Some(Pulse::new(self.label.clone(), !memory.values().all(|s| *s)))
            }
        }
    }
}

pub type Data = HashMap<String, Module>;

fn step_modules(mut data: Data) -> (Data, (usize, usize)) {
    let mut pulses = VecDeque::from([("broadcaster".to_owned(), Pulse::new("broadcaster".to_owned(), false))]);
    let mut low = 0;
    let mut high = 0;
    while let Some((to, pulse)) = pulses.pop_front() {
        if pulse.pulse { high += 1; } else { low += 1; }
        let Some(module) = data.get_mut(&to) else { continue; };
        if let Some(new_pulse) = module.process_signal(pulse) {
            for dest in module.destinations.iter().cloned() {
                pulses.push_back((dest.clone(), new_pulse.clone()));
            }
        }
    }
    
    (data, (low, high))
}

fn step_modules_2(mut data: Data) -> (Data, Vec<Pulse>) {
    let mut pulses = VecDeque::from([("broadcaster".to_owned(), Pulse::new("broadcaster".to_owned(), false))]);
    
    let mut all = vec![];
    while let Some((to, pulse)) = pulses.pop_front() {
        let Some(module) = data.get_mut(&to) else { continue; };
        if let Some(new_pulse) = module.process_signal(pulse) {
            all.push(new_pulse.clone());
            for dest in module.destinations.iter().cloned() {
                pulses.push_back((dest.clone(), new_pulse.clone()));
            }
        }
    }

    (data, all)
}


impl Day<Data> for Day20 {
    fn parse_file(&self, file_content: String) -> Data {
        let mut conjunctions = vec![];
        
        let mut modules = file_content.lines().map(|s| {
            let (label, destinations) = s.split_once(" -> ").unwrap();
            let (label, m_type) = match label {
                "broadcaster" => {("broadcaster".to_owned(), ModuleType::Broadcaster)},
                x => {
                    if &x[0..1] == "%" {
                        (x[1..].to_owned(), ModuleType::FlipFlop { memory: false })
                    } else {
                        (x[1..].to_owned(), ModuleType::Conjunction { memory: HashMap::new() })
                    }
                }
            };
            
            if matches!(m_type, ModuleType::Conjunction { .. }) {
                conjunctions.push(label.clone());
            }
            
            let destinations = destinations.split(", ")
                .map(str::to_owned)
                .collect();
            
            (label.clone(), Module {
                m_type,
                label,
                destinations
            })
        }).collect::<HashMap<_, _>>();
        
        let mut pairs = vec![];
        for module in modules.values() {
            for m in module.destinations.iter().filter(|s| conjunctions.contains(s)) {
                pairs.push((m.clone(), module.label.clone()));
            }
        }
        
        for (conj, module) in pairs {
            if let ModuleType::Conjunction { memory } = &mut modules.get_mut(&conj).unwrap().m_type {
                memory.insert(module, false);
            }
        }
        
        modules
    }

    fn part_1(&self, data: &Data) -> i64 {
        let mut data = data.clone();
        
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (l, h);
            (data, (l, h)) = step_modules(data);
            low += l;
            high += h;
        }

        (low * high) as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        if !data.values().any(|s| s.destinations.contains(&"rx".to_owned())) { return 0; }
        
        let mut receivers: HashMap<String, HashSet<String>> = HashMap::new();
        for (name, module) in data.iter() {
            for destination in module.destinations.iter() {
                receivers.entry(destination.clone())
                    .and_modify(|x| { x.insert(name.clone()); })
                    .or_insert(HashSet::from([name.clone()]));
            }
        }

        for x in data.values() {
            for y in x.destinations.iter() {
                println!("{} {}", x.label_fmt(), data.get(y).map(|s| s.label_fmt()).unwrap_or(y.clone()));
            }
        }
        
        let mut record = vec![];
        let mut data = data.clone();
        for _ in 0..50 {
            let step;
            (data, step) = step_modules_2(data);
            record.push(step);
        }
        
        print!("  ");
        for t in 0..record.len() {
            print!("{: ^4}", t);
        }
        println!();
        
        let mut modules = data.keys().cloned().collect_vec();
        modules.sort_unstable();
        
        for module in &modules {
            print!("{}", module[0..2].to_owned());
            for t in 0..record.len() {
                let num_high = record[t].iter().filter(|p| &p.from == module && p.pulse).count();
                let num_low = record[t].iter().filter(|p| &p.from == module && !p.pulse).count();
                print!("{}{}",
                    format!("{: >2}", num_low).blue(),
                    format!("{: <2}", num_high).red(),
                );
                // let num_low = record[t]
            }
            println!();
        }
        
        panic!("Exited without finding rx?")
    }
}