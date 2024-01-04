
pub struct Day20;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::{thread, time};
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
    pub fn try_make_cycle(&self, receivers: &HashMap<String, HashSet<String>>, name_to_cycle: &HashMap<String, PulseCycles2>) -> Option<PulseCycles2> {
        self.can_make_cycle(receivers, name_to_cycle)
            .then(|| {
                match self.m_type {
                    ModuleType::Broadcaster => PulseCycles2::broadcaster(),
                    ModuleType::FlipFlop { .. } => PulseCycles2::try_make_flip_flop(
                        &receivers.get(&self.label).unwrap().iter()
                            .map(|s| name_to_cycle.get(s).unwrap().clone())
                            .collect_vec()
                    ).unwrap(),
                    ModuleType::Conjunction { .. } => PulseCycles2::merge_to_conjunction(
                        &receivers.get(&self.label).unwrap().iter()
                            .map(|s| (s.clone(), name_to_cycle.get(s).unwrap().clone()))
                            .collect_vec()).unwrap()
                }
                // todo!()
            })
    }
    
    pub fn can_make_cycle(&self, receivers: &HashMap<String, HashSet<String>>, name_to_cycle: &HashMap<String, PulseCycles2>) -> bool {
        match self.m_type {
            ModuleType::Broadcaster => true,
            ModuleType::FlipFlop { .. } => {
                receivers.get(&self.label).unwrap()
                    .iter().all(|s| name_to_cycle.get(s).unwrap_or(&PulseCycles2::default()).lows.is_some())
            }
            ModuleType::Conjunction { .. } => {
                receivers.get(&self.label).unwrap()
                    .iter().all(|s| name_to_cycle.get(s).unwrap_or(&PulseCycles2::default()).has_both_cycles())
            }
        }
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

fn step_modules_2(mut data: Data) -> (Data, bool) {
    let mut pulses = VecDeque::from([("broadcaster".to_owned(), Pulse::new("broadcaster".to_owned(), false))]);
    while let Some((to, pulse)) = pulses.pop_front() {
        let Some(module) = data.get_mut(&to) else { 
            if to == "rx" && !pulse.pulse {
                return (data, true);
            }
            continue;
        };
        if let Some(new_pulse) = module.process_signal(pulse) {
            for dest in module.destinations.iter().cloned() {
                pulses.push_back((dest.clone(), new_pulse.clone()));
            }
        }
    }

    (data, false)
}

fn step_modules_3(mut data: Data) -> Vec<String> {
    const FRACTION: f32 = 1.0;
    let min = (data.len() as f32 * FRACTION) as usize;
    let mut first_pulses = vec![];
    let mut pulses = VecDeque::from([("broadcaster".to_owned(), Pulse::new("broadcaster".to_owned(), false))]);
    while let Some((to, pulse)) = pulses.pop_front() {
        if !first_pulses.contains(&to) { 
            first_pulses.push(to.clone());
            if first_pulses.len() >= min {
                return first_pulses;
            }
        }
        let Some(module) = data.get_mut(&to) else {
            if to == "rx" && !pulse.pulse {
                return first_pulses;
            }
            continue;
        };
        if let Some(new_pulse) = module.process_signal(pulse) {
            for dest in module.destinations.iter().cloned() {
                pulses.push_back((dest.clone(), new_pulse.clone()));
            }
        }
    }

    first_pulses
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
                println!("{} {}", x.label, y);
            }
        }
        
        
        
        // let x: HashMap<String, PulseCycle> = HashMap::from([("broadcaster".to_owned(), PulseCycle::broadcaster())]);
        let mut name_to_cycle = data.keys().map(|s| (s.clone(), PulseCycles2::default())).collect();
        let mut to_visit = vec![data.get(&String::from("broadcaster")).unwrap().clone()];
        while let Some(module) = to_visit.pop() {
            println!("{}", module.label);
            if let Some(cycle) = module.try_make_cycle(&receivers, &name_to_cycle) {
                println!("success! {:?}", cycle);
                if module.label == "rx" && cycle.lows.is_some() {
                    return cycle.lows.unwrap().times[0] as i64;
                }
                
                name_to_cycle.insert(module.label.clone(), cycle);
                to_visit.extend(module.destinations.iter().map(|r| data.get(r).unwrap().clone()))
            }
        }
        // 
        // *name_to_cycle.get(&String::from("rx"))
        //     .and_then(|x| x.lows.first())
        //     .unwrap() as i64
        
        // todo!()
        panic!("Exited without finding rx?")
    }
}

fn print_data(data: &Data) {
    print!("\x1B[2J\x1B[1;1H");
    println!("-------------");
    let mut modules = data.keys().cloned().collect::<Vec<_>>();
    modules.sort_unstable();
    
    for module in modules {
        let module = data.get(&module).unwrap();
        println!("{}", module);
    }
    thread::sleep(time::Duration::from_millis(100));
}

fn print_data_subset(data: &Data, modules: &Vec<String>) {
    print!("\x1B[2J\x1B[1;1H");
    println!("-------------");
    for module in modules {
        let Some(module) = data.get(module) else { continue; };
        println!("{}", module);
    }
    thread::sleep(time::Duration::from_millis(500));
}


#[derive(Eq, PartialEq, Clone, Debug)]
struct PulseCycle {
    length: usize,
    highs: Vec<usize>,
    lows: Vec<usize>,
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct PulseCycles2 {
    highs: Option<Cycle>,
    lows: Option<Cycle>,
}

impl PulseCycles2 {
    pub fn broadcaster() -> Self {
        Self {
            highs: None,
            lows: Some(Cycle {
                length: 1,
                times: vec![0],
            }),
        }
    }
    
    pub fn try_get_cycle_length(&self) -> Option<usize> {
        match (&self.highs, &self.lows) {
            (Some(Cycle {length: a, ..}), Some(Cycle {length: b, ..})) => Some(a.lcm(b)),
            (_, _) => None
        }
    }
    
    pub fn try_make_flip_flop(parents: &Vec<Self>) -> Option<Self> {
        parents.iter().all(|x| x.lows.is_some())
            .then(|| {
                let mut cycle = parents.iter().map(|x| x.lows.clone().unwrap())
                    .reduce(|a, b| a.merge(&b)).unwrap();
                if cycle.length.is_odd() { cycle = cycle.repeat(2); }
                let length = cycle.length;
                let (highs, lows) = cycle.times.chunks_exact(2).map(|x| (x[0], x[1])).unzip();
                
                Self {
                    highs: Some(Cycle { length, times: highs }),
                    lows: Some(Cycle { length, times: lows }),
                }
            })
    }

    pub fn try_to_pulses(&self, name: &String) -> Option<Vec<(Pulse, usize)>> {
        let length = self.try_get_cycle_length()?;
        let highs = self.highs.clone()
            .map(|highs| highs.repeat(length / highs.length))?;
        let lows = self.lows.clone()
            .map(|lows| lows.repeat(length / lows.length))?;
        Some(highs.times.iter()
            .map(|t| (Pulse { from: name.clone(), pulse: true }, *t))
            .merge_by(
                lows.times.iter()
                    .map(|t| (Pulse { from: name.clone(), pulse: false }, *t)),
                |a, b| a.1 >= b.1
            ).collect())
    }
    
    pub fn merge_to_conjunction(parents: &Vec<(String, Self)>) -> Option<Self> {
        let cycle_length = parents.iter()
            .map(|r| r.1.try_get_cycle_length().unwrap())
            .reduce(|a, b| a.lcm(&b)).unwrap();
        let mut x = parents.iter()
            .flat_map(|(name, cycle)| cycle.repeat_to_length(cycle_length).try_to_pulses(name).unwrap())
            .collect_vec();
        x.sort_by_key(|a| a.1);
    
        let mut memory = parents.iter().map(|s| {
            (s.0.clone(), false)
        }).collect::<HashMap<_, _>>();
    
        let mut highs = vec![];
        let mut lows = vec![];
    
        for (Pulse { from, pulse }, t) in x {
            memory.insert(from, pulse);
            let output = memory.values().all(|x| x == &true);
            (if output { &mut highs } else { &mut lows }).push(t);
        }
    
        Some(Self {
            highs: Some(Cycle::new(cycle_length, highs)),
            lows: Some(Cycle::new(cycle_length, lows)),
        })
    }
    
    pub fn repeat_to_length(&self, length: usize) -> Self {
        Self {
            highs: self.highs.clone().map(|s| s.repeat(length / s.length)),
            lows: self.lows.clone().map(|s| s.repeat(length / s.length)),
        }
    }
    
    pub fn has_both_cycles(&self) -> bool {
        self.highs.is_some() && self.lows.is_some()
    }
    
    // pub fn make_flip_flop
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Cycle {
    length: usize,
    times: Vec<usize>
}

impl Cycle {
    pub fn new(length: usize, times: Vec<usize>) -> Self {
        Self {
            length,
            times,
        }
    }
    
    pub fn merge(&self, other: &Self) -> Self {
        let length = self.length.lcm(&other.length);
        let c1 = self.repeat(length / self.length);
        let c2 = other.repeat(length / other.length);
        
        Self {
            length,
            times: c1.times.into_iter().merge(c2.times).collect_vec(),
        }
    }
    
    pub fn repeat(&self, n: usize) -> Self {
        Self {
            length: self.length * n,
            times: (0..n).flat_map(move |r|
                (0..self.times.len()).map(move |i| self.times[i] + r * self.length))
                .collect_vec(),
        }
    }
    
    pub fn to_pulses(&self, name: &String, pulse: bool) -> Vec<(Pulse, usize)> {
        self.times.iter()
            .map(|t| (Pulse { from: name.clone(), pulse }, *t))
            .collect()
    }
}

impl PulseCycle {
    pub fn broadcaster() -> Self {
        Self {
            length: 1,
            highs: vec![],
            lows: vec![0],
        }
    }

    pub fn merge_to_flip_flop(parents: Vec<Self>) -> Self {
        let total = parents.into_iter().reduce(PulseCycle::merge).unwrap();
        total.to_flip_flop()
    }

    pub fn to_flip_flop(&self) -> Self {
        let (lows, length) = if self.lows.len().is_odd()
        { (self.repeat(2).lows, self.length * 2) }
        else { (self.lows.clone(), self.length) };

        let (highs, lows) = lows.chunks_exact(2).map(|x| (x[0], x[1])).unzip();
        Self {
            length,
            highs,
            lows,
        }
    }

    pub fn merge_to_conjunction(parents: &Vec<(String, PulseCycle)>) -> Self {
        let cycle_length = parents.iter()
            .map(|r| r.1.length)
            .reduce(|a, b| a.lcm(&b)).unwrap();
        let mut x = parents.iter()
            .flat_map(|(name, cycle)| cycle.repeat(cycle_length / cycle.length).to_pulses(name))
            .collect_vec();
        x.sort_by_key(|a| a.1);
        
        let mut memory = parents.iter().map(|s| {
            (s.0.clone(), false)
        }).collect::<HashMap<_, _>>();
        
        let mut highs = vec![];
        let mut lows = vec![];
        
        for (Pulse { from, pulse }, t) in x {
            memory.insert(from, pulse);
            let output = memory.values().all(|x| x == &true);
            (if output { &mut highs } else { &mut lows }).push(t);
        }
        
        Self {
            length: cycle_length,
            highs,
            lows,
        }
    }

    pub fn merge(self, other: Self) -> Self {
        let total_length = self.length.lcm(&other.length);
        let num_repeats_self = total_length / self.length;
        let num_repeats_other = total_length / other.length;

        let a = self.repeat(num_repeats_self);
        let b = other.repeat(num_repeats_other);

        Self {
            length: total_length,
            highs: a.highs.into_iter().merge(b.highs.into_iter()).collect_vec(),
            lows: a.lows.into_iter().merge(b.lows.into_iter()).collect_vec(),
        }
    }
    
    fn to_pulses(self, name: &String) -> Vec<(Pulse, usize)> {
        let mut lows = self.lows.into_iter()
            .map(|x| (Pulse { from: name.clone(), pulse: false }, x))
            .collect_vec();
        let mut highs = self.highs.into_iter()
            .map(|x| (Pulse { from: name.clone(), pulse: true }, x))
            .collect_vec();
        
        let mut x = lows;
        x.append(&mut highs);
        x.sort_by_key(|a| a.1);
        x
    }

    fn repeat(&self, n: usize) -> Self {
        fn repeat_cycle(vec: &Vec<usize>, length: usize, n: usize) -> Vec<usize> {
            let mut out = Vec::with_capacity(vec.len() * n);

            for r in 0..n {
                for i in 0..vec.len() {
                    out.push(vec[i] + r * length);
                }
            }

            out
        }

        Self {
            length: self.length * n,
            highs: repeat_cycle(&self.highs, self.length, n),
            lows: repeat_cycle(&self.lows, self.length, n),
        }
    }
}

#[test]
fn test_pulse_cycle() {
    let t = PulseCycle::broadcaster();
    assert_eq!(t.repeat(2), PulseCycle {
        length: 2,
        highs: vec![],
        lows: vec![0, 1],
    });

    assert_eq!(t.to_flip_flop(), PulseCycle {
        length: 2,
        highs: vec![0],
        lows: vec![1],
    });

    assert_eq!(t.to_flip_flop().to_flip_flop(), PulseCycle {
        length: 4,
        highs: vec![1],
        lows: vec![3],
    });
    
    dbg!(PulseCycle::merge_to_conjunction(&Vec::from([("a".to_owned(), PulseCycle {
        length: 5,
        highs: vec![3],
        lows: vec![1,3],
    }),
        ("b".to_owned(), PulseCycle {
            length: 4,
            highs: vec![1,2],
            lows: vec![0,3],
        })
    ])));
}