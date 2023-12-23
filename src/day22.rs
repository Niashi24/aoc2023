use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use crate::day::Day;
use crate::ranges::{min_max_comp, RangeD};

pub struct Day22;

pub type Data = Vec<RangeD<3>>;

impl Day<Data> for Day22 {
    fn parse_file(&self, file_content: String) -> Data {
        file_content.lines().map(|s| {
            let (start, end) = s.split_once("~").unwrap();
            let start = start.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .try_into().unwrap();
            let end = end.split(",")
                .map(|s| s.parse::<usize>().unwrap() + 1)
                .collect::<Vec<_>>()
                .try_into().unwrap();
            
            RangeD {
                start,
                end
            }
        }).collect()
    }

    fn part_1(&self, data: &Data) -> i64 {
        let mut blocks = data.clone();
        for block in blocks.iter() {
            println!("{}", block);
        }
        println!();
        
        blocks.sort_unstable_by_key(|b| b.start[2]);
        let mut grid_map = HashMap::new();
        
        for i in 0..blocks.len() {
            let mut block = blocks.get(i).unwrap().clone();
            while !block.iter().any(|p| grid_map.contains_key(&p)) && &block.start[2] != &0 {
                block.offset_component_neg(2, 1);
            }
            block.offset_component(2, 1);
            
            for p in block.iter() {
                grid_map.insert(p, i);
            }
        }
        
        let (mins, maxes) = min_max_comp(grid_map.keys().cloned()).unwrap();
        for x in mins[0]..=maxes[0] {
            println!("\nx={x}");
            for z in (mins[2]..=maxes[2]).rev() {
                for y in mins[1]..=maxes[1] {
                    print!("{}", grid_map.get(&[x,y,z]).map(|i| i.to_string())
                        .unwrap_or(".".to_string()));
                }
                println!();
            }
        }
        
        let mut supports = HashMap::<usize, HashSet<usize>>::new();
        
        for (pos, id) in grid_map.iter() {
            let pos = [pos[0], pos[1], pos[2] - 1];
            if let Some(i) = grid_map.get(&pos) {
                if i == id { continue; }
                
                supports.entry(*id)
                    .and_modify(|x| { x.insert(*i); })
                    .or_insert(HashSet::from([*i]));
            }
        }
        
        // let x = supports.values().filter(|s| s.len() != 1).count();
        let mut supported_blocks: HashSet<_, RandomState> = HashSet::from_iter((0..blocks.len()));

        for x in supports.values().filter_map(|s| { 
            if s.len() == 1 {
                Some(s.iter().next().unwrap())
            } else {
                None
            }
        }) {
            supported_blocks.remove(x);
        }
        
        // for block in blocks.iter() {
        //     println!("{}", block);
        // }
        
        // let mut supported_blocks: HashSet<RangeD<3>, RandomState> = HashSet::from_iter(blocks.iter().cloned());
        // 
        // for (i, b) in blocks.iter().enumerate() {
        //     let mut block = b.clone();
        //     
        //     let supporting_blocks = blocks[..i.saturating_sub(1)]
        //         .iter().filter(|r| block.intersects(r)).chain(blocks[i+1..]
        //         .iter().filter(|r| block.intersects(r))).collect::<Vec<_>>();
        //     
        //     if supporting_blocks.len() == 1 {
        //         supported_blocks.remove(supporting_blocks.first().unwrap());
        //     }            
        // }
        // 
        // dbg!(&supported_blocks);
        
        supported_blocks.len() as i64
    }

    fn part_2(&self, data: &Data) -> i64 {
        let mut blocks = data.clone();
        for block in blocks.iter() {
            println!("{}", block);
        }
        println!();

        blocks.sort_unstable_by_key(|b| b.start[2]);
        let mut grid_map = HashMap::new();

        for i in 0..blocks.len() {
            let mut block = blocks.get(i).unwrap().clone();
            while !block.iter().any(|p| grid_map.contains_key(&p)) && &block.start[2] != &0 {
                block.offset_component_neg(2, 1);
            }
            block.offset_component(2, 1);

            for p in block.iter() {
                grid_map.insert(p, i);
            }
        }

        let (mins, maxes) = min_max_comp(grid_map.keys().cloned()).unwrap();
        for x in mins[0]..=maxes[0] {
            println!("\nx={x}");
            for z in (mins[2]..=maxes[2]).rev() {
                for y in mins[1]..=maxes[1] {
                    print!("{}", grid_map.get(&[x,y,z]).map(|i| i.to_string())
                        .unwrap_or(".".to_string()));
                }
                println!();
            }
        }

        let mut supports = HashMap::<usize, HashSet<usize>>::new();

        for (pos, id) in grid_map.iter() {
            let pos = [pos[0], pos[1], pos[2] - 1];
            if let Some(i) = grid_map.get(&pos) {
                if i == id { continue; }

                supports.entry(*id)
                    .and_modify(|x| { x.insert(*i); })
                    .or_insert(HashSet::from([*i]));
            }
        }

        // let x = supports.values().filter(|s| s.len() != 1).count();
        let mut supported_blocks: HashSet<_, RandomState> = HashSet::from_iter((0..blocks.len()));
        
        dbg!(&supports);
        
        let mut s = HashSet::new();

        for x in supports.values().filter_map(|s| {
            if s.len() == 1 {
                Some(s.iter().next().unwrap())
            } else {
                None
            }
        }) {
            s.insert(x);
            supported_blocks.remove(x);
        }
        
        dbg!(&supported_blocks);
        dbg!(&s);
        
        let mut sum = 0;
        for base_blocks in s {
            dbg!(base_blocks);
            let mut to_remove = vec![*base_blocks];
            let mut supports = supports.clone();
            let mut removed = HashSet::new();
            while let Some(b) = to_remove.pop() {
                removed.insert(b);
                for (id, set) in supports.iter_mut()
                    .filter(|(i, _)| !removed.contains(i)) {
                    set.remove(&b);
                    if set.len() == 0 {
                        to_remove.push(*id);
                    }
                }
                
                // dbg!(b, &supports);
            }
            sum += removed.len() - 1;
        }
        
        sum as i64

        // for block in blocks.iter() {
        //     println!("{}", block);
        // }

        // let mut supported_blocks: HashSet<RangeD<3>, RandomState> = HashSet::from_iter(blocks.iter().cloned());
        // 
        // for (i, b) in blocks.iter().enumerate() {
        //     let mut block = b.clone();
        //     
        //     let supporting_blocks = blocks[..i.saturating_sub(1)]
        //         .iter().filter(|r| block.intersects(r)).chain(blocks[i+1..]
        //         .iter().filter(|r| block.intersects(r))).collect::<Vec<_>>();
        //     
        //     if supporting_blocks.len() == 1 {
        //         supported_blocks.remove(supporting_blocks.first().unwrap());
        //     }            
        // }
        // 
        // dbg!(&supported_blocks);

        // supported_blocks.len() as i64
    }
}