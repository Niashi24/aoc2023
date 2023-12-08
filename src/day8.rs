use std::collections::HashMap;
use std::iter::{Cycle, Enumerate};
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
        #[derive(Clone)]
        struct Pos<'a>((&'a Node, &'a (Node, Node)), Cycle<Enumerate<Iter<'a, Direction>>>, usize);
        impl PartialEq for Pos<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.2 == other.2
            }
        }

        fn successor<'a>(pos: Pos<'a>, nodes: &'a HashMap<Node, (Node, Node)>) -> Pos<'a> {
            let Pos(cur, mut dir, _) = pos;
            let (index, d) = dir.next().unwrap();
            let next = match d {
                Direction::Left => &cur.1.0,
                Direction::Right => &cur.1.1
            };
            Pos((next, nodes.get(next).unwrap()), dir, index)
        }

        fn gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { gcd(b, a % b) }}

        data.nodes.iter()
            .filter(|(cur, _)| cur.0[2] == 'A')
            .map(|x| Pos(x, data.directions.iter().enumerate().cycle(), 0))
            .map(|x| brent(x, |x| successor(x, &data.nodes)))
            .map(|(cycle_length, mut cycle_start, cycle_start_index)|
                (cycle_start_index..cycle_start_index + cycle_length)
                    .fold((cycle_start, vec![]), |(n, mut zs), i| {
                    if n.0.0.0[2] == 'Z' {
                        zs.push((n.2, i));
                    }
                    (successor(n, &data.nodes), zs)
                }).1
            ).reduce(|mut x, mut y| {
                x.retain(|(x, _)| y.iter().any(|(y,_)| x == y));
                y.retain(|(y, _)| x.iter().any(|(x, _)| x == y));
                x.append(&mut y);
                x
            }).unwrap().into_iter()
            .map(|x| x.1)
            .reduce(|x, y| x * y / gcd(x, y)).unwrap() as i64
    }
}