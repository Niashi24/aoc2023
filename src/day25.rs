use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;
use itertools::Itertools;
use pathfinding::prelude::{bfs, kruskal, separate_components, strongly_connected_components};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::combinations::CombinationIterator;
use crate::day::Day;
use crate::ranges::RangeD;

pub struct Day25;

#[derive(Clone)]
pub struct Component {
    name: String,
    connections: Vec<String>
}

impl Component {
    pub fn to_group(self) -> Vec<String> {
        let mut connections = self.connections;
        connections.insert(0, self.name);
        connections
    }
    
    pub fn to_range(&self) -> Range<usize> {
        1..(self.connections.len() + 1)
    }
}

impl Day<Vec<Component>> for Day25 {
    fn parse_file(&self, file_content: String) -> Vec<Component> {
        file_content.lines()
            .map(|s| {
                let (name, connections) = s.split_once(": ").unwrap();
                Component {
                    name: name.to_owned(),
                    connections: connections.split_whitespace()
                        .map(str::to_string).collect()
                }
            }).collect()
    }

    fn part_1(&self, data: &Vec<Component>) -> i64 {
        
        // turn components into node to connections map
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for x in data.clone() {
            for y in x.connections {
                map.entry(x.name.clone())
                    .and_modify(|s| s.push(y.clone()))
                    .or_insert(vec![y.clone()]);
                
                map.entry(y)
                    .and_modify(|s| s.push(x.name.clone()))
                    .or_insert(vec![x.name.clone()]);
            }
        }
        
        // find and remove triple edges
        // just choose random nodes until the triple edges are contained between start and end
        // kinda jank but nodes are ~50% distributed between so should eventually get a valid pair
        let mut start = data.choose(&mut thread_rng()).unwrap().name.clone();
        let mut end = data.choose(&mut thread_rng()).unwrap().name.clone();
        while end == start || // having them be the same causes infinite loop
            remove_triple_edges(&mut map, &start, &end).is_err() {  // removal failed
            end = data.choose(&mut thread_rng()).unwrap().name.clone();
            start = data.choose(&mut thread_rng()).unwrap().name.clone();
        }        
        
        // find size of one of the groups
        let mut to_visit = vec![start.clone()];
        let mut visited = HashSet::from([start]);
        while let Some(pos) = to_visit.pop() {
            map.get(&pos)
                .map(|sucs| sucs.iter().for_each(|x| {
                    if visited.contains(x) { return; }
                    visited.insert(x.clone());
                    to_visit.push(x.clone());
                }));
        }
        
        (visited.len() * (map.len() - visited.len())) as i64
    }

    fn part_2(&self, data: &Vec<Component>) -> i64 {
        0
    }
}

fn add_edge<N: Eq + Hash + Clone>(map: &mut HashMap<N, Vec<N>>, a: N, b: N) {
    map.get_mut(&a).map(|s| s.push(b.clone()));
    map.get_mut(&b).map(|s| s.push(a));
}

fn remove_edge<N: Eq + Hash + Clone + Debug>(map: &mut HashMap<N, Vec<N>>, a: &N, b: &N) -> (N, N) {
    // dbg!(a, b);
    (map.get_mut(&a).and_then(|s| s.iter().position(|x| x == b).map(|i| s.swap_remove(i))).unwrap(),
    map.get_mut(&b).and_then(|s| s.iter().position(|x| x == a).map(|i| s.swap_remove(i))).unwrap())
}

// finds and removes the three edges that divides the components
// returns Err if the three edges are not between start and end
// nothing is removed or added if it returns Err
fn remove_triple_edges(map: &mut HashMap<String, Vec<String>>, start: &String, end: &String) -> Result<(), String> {
    let mut temp_map = map.clone();
    let paths = (0..3).map(|_| {
        let path = bfs(
            start,
            |s| temp_map.get(s).unwrap().clone(),
            |s| s == end,
        ).unwrap();
        
        for x in path.windows(2) {
            remove_edge(&mut temp_map, &x[0], &x[1]);
        }

        path
    }).collect_vec();

    let x = paths.iter().map(|i| 1..i.len() - 1).collect_vec();
    let r = RangeD::<3>::from_range_1d(x.try_into().unwrap());
    for [i, j, k] in &r {
        let s_1 = &paths[0][i];
        let e_1 = &paths[0][i + 1];
        let s_2 = &paths[1][j];
        let e_2 = &paths[1][j + 1];
        let s_3 = &paths[2][k];
        let e_3 = &paths[2][k + 1];

        let (s1, e1) = remove_edge(map, s_1, e_1);
        let (s2, e2) = remove_edge(map, s_2, e_2);
        let (s3, e3) = remove_edge(map, s_3, e_3);
        
        // println!("{} {} {} {} {} {}", s1, e1, s2, e2, s3, e3);

        if bfs(
            start,
            |s| map.get(s).unwrap().clone(),
            |s| s == end,
        ).is_none() {
            return Ok(());
        }
        
        add_edge(map, s1, e1);
        add_edge(map, s2, e2);
        add_edge(map, s3, e3);
        
        // println!("{:?}", &map);
    }
    
    Err("Start and end nodes are within the same part".to_owned())
}