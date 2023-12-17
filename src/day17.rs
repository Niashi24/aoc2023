use pathfinding::prelude::dijkstra;
use crate::day::Day;
use crate::grid::Grid;

pub struct Day17;

pub type Data = Grid<usize>;

#[derive(Eq, PartialEq, Hash, Clone)]
#[derive(Debug)]
pub struct Pos {
    pos: (i64, i64),
    dir: (i64, i64),
    time_in_dir: i64,
}

impl Pos {
    fn turn_left_and_step(&self) -> Self {
        let new_dir = (-self.dir.1, self.dir.0);
        let new_pos = (self.pos.0 + new_dir.0, self.pos.1 + new_dir.1);
        Pos {
            pos: new_pos,
            dir: new_dir,
            time_in_dir: 1,
        }
    }

    fn turn_right_and_step(&self) -> Self {
        let new_dir = (self.dir.1, -self.dir.0);
        let new_pos = (self.pos.0 + new_dir.0, self.pos.1 + new_dir.1);
        Pos {
            pos: new_pos,
            dir: new_dir,
            time_in_dir: 1,
        }
    }

    fn move_forward(&self) -> Self {
        let new_dir = self.dir;
        let new_pos = (self.pos.0 + new_dir.0, self.pos.1 + new_dir.1);
        Pos {
            pos: new_pos,
            dir: new_dir,
            time_in_dir: self.time_in_dir + 1,
        }
    }
}

impl Day<Data> for Day17 {
    fn parse_file(&self, file_content: String) -> Data {
        Data::new(file_content.lines()
            .map(|s| s.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect())
            .collect())
    }

    fn part_1(&self, data: &Data) -> i64 {
        let start = Pos {
            pos: (0, 0),
            dir: (1, 0),
            time_in_dir: 0,
        };

        fn successors(node: &Pos, grid: &Data) -> Vec<(Pos, usize)> {
            let cost_fn = |p: Pos| {
                let cost = *grid.get_i(p.pos.0, p.pos.1).unwrap_or(&1000);
                (p, cost)
            };
            
            if node.time_in_dir <= 2 {
                let forward = node.move_forward();
                let left = node.turn_left_and_step();
                let right = node.turn_right_and_step();
                [forward, left, right]
                    .into_iter()
                    .map(cost_fn).collect()
            } else {
                let left = node.turn_left_and_step();
                let right = node.turn_right_and_step();
                [left, right]
                    .into_iter()
                    .map(cost_fn).collect()
            }
        }

        dijkstra(&start,
         |x| successors(x, &data),
         |x| x.pos.0 == (data.w - 1) as i64 
             && x.pos.1 == (data.h - 1) as i64
        ).unwrap().1 as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        let start = Pos {
            pos: (0, 0),
            dir: (1, 0),
            time_in_dir: 0,
        };

        fn successors(node: &Pos, grid: &Data) -> Vec<(Pos, usize)> {
            let cost_fn = |p: Pos| {
                let cost = *grid.get_i(p.pos.0, p.pos.1).unwrap_or(&1000);
                (p, cost)
            };

            if (1..=3).contains(&node.time_in_dir) {
                [node.move_forward()]
                    .into_iter()
                    .map(cost_fn)
                    .collect()
            } else if node.time_in_dir == 10 {
                let left = node.turn_left_and_step();
                let right = node.turn_right_and_step();
                [left, right]
                    .into_iter()
                    .map(cost_fn)
                    .collect()
            } else {
                let forward = node.move_forward();
                let left = node.turn_left_and_step();
                let right = node.turn_right_and_step();
                [forward, left, right]
                    .into_iter()
                    .map(cost_fn)
                    .collect()
            }
        }

        dijkstra(&start,
                 |x| successors(x, &data),
                 |x| x.pos.0 == (data.w - 1) as i64
                     && x.pos.1 == (data.h - 1) as i64
                     && x.time_in_dir > 3,
        ).unwrap().1 as i64
    }
}