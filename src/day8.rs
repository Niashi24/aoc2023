use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::{Cycle, Enumerate};
use std::ops::Rem;
use std::slice::Iter;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use pathfinding::directed::cycle_detection::brent;
use crate::day::Day;

pub struct Day8;

pub struct Data {
    directions: Vec<Direction>,
    nodes: HashMap<Node, (Node, Node)>
}

#[derive(Eq, PartialEq, Clone, Hash)]
#[derive(Debug)]
pub struct Node([char; 3]);

const START: Node = Node(['A', 'A', 'A']);
const END: Node = Node(['Z', 'Z', 'Z']);

#[derive(Eq, PartialEq, Clone, Hash)]
pub enum Direction {Left, Right}

impl Day<Data> for Day8 {
    fn parse_file(&self, file_content: String) -> Data {
        let mut lines = file_content.lines();
        let directions = lines.next().unwrap().chars().map(|x| match x {
            'R' => Direction::Right, 'L' => Direction::Left, x => panic!("{}", x)
        }).collect();

        let nodes = lines.skip(1).map(|x| {
            let cur = (&x[0..3]).chars().collect::<Vec<_>>().try_into().unwrap();
            let left = (&x[7..10]).chars().collect::<Vec<_>>().try_into().unwrap();
            let right = (&x[12..15]).chars().collect::<Vec<_>>().try_into().unwrap();
            (Node(cur), (Node(left), Node(right)))
        }).collect();

        Data {
            directions,
            nodes
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.directions.iter().cycle().fold_while((0, &START), |(i, cur), d| {
            if cur == &END { return Done((i, cur)); }
            let (left, right) = data.nodes.get(cur).unwrap();
            let next = match d {
                Direction::Left => left,
                Direction::Right => right
            };

            Continue((i + 1, next))
        }).into_inner().0
    }

    fn part_2(&self, data: &Data) -> i64 {
        #[derive(Eq, PartialEq, Hash, Clone)]
        struct Pos<'a>((&'a Node, &'a (Node, Node)), usize);

        fn successor<'a>(pos: Pos<'a>, nodes: &'a HashMap<Node, (Node, Node)>, directions: &'a Vec<Direction>) -> Pos<'a> {
            let Pos(cur, i) = pos;
            let dir = directions.get(i).unwrap();
            let next = match dir {
                Direction::Left => &cur.1.0,
                Direction::Right => &cur.1.1
            };
            Pos((next, nodes.get(next).unwrap()), (i+1).rem(directions.len()))
        }

        fn gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { gcd(b, a % b) }}

        data.nodes.iter()
            .filter(|(cur, _)| cur.0[2] == 'A')
            .map(|x| Pos(x, 0))
            .map(|x| brent(x, |x| successor(x, &data.nodes, &data.directions)))
            .map(|(cycle_length, mut cycle_start, cycle_start_index)|
                (cycle_start_index..cycle_start_index + cycle_length)
                    .fold((cycle_start, vec![]), |(n, mut zs), i| {
                    if n.0.0.0[2] == 'Z' {
                        zs.push((cycle_length, (n.1, i)));
                    }
                    (successor(n, &data.nodes, &data.directions), zs)
                }).1
            ).reduce(|mut x, mut y| {
                x.retain(|(_, (x, _))| y.iter().any(|(_, (y, _))| x == y));
                y.retain(|(_, (y, _))| x.iter().any(|(_, (x, _))| x == y));
                x.append(&mut y);
                x
            }).unwrap().into_iter()
            // map to index where it lands on the Z
            .map(|x| x.1.1)
            // get lcm of
            .reduce(|x, y| x * y / gcd(x, y)).unwrap() as i64
    }
}