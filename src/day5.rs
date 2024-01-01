use std::cmp::Ordering;
use std::ops::Range;
use itertools::Itertools;
use crate::day::Day;

pub struct Day5;

pub struct Info {
    seeds: Vec<i64>,
    maps: Vec<Map>
}

impl Info {
    #[inline]
    pub fn transform(&self, n: i64) -> i64 {
        self.maps.iter().fold(n, |n, m| { m.transform(n) })
    }
    
    #[inline]
    pub fn inv_transform(&self, n: i64) -> i64 {
        self.maps.iter().rev().fold(n, |n, m| m.inv_transform(n))
    }
}

#[derive(Debug)]
pub struct Map {
    ranges: Vec<(Range<i64>, i64)>
}

impl Map {
    #[inline]
    pub fn transform(&self, n: i64) -> i64 {
        n + self.ranges.iter().find(|(r, _)| r.contains(&n)).map(|(_, c)| c).unwrap_or(&0)
    }
    
    #[inline]
    pub fn inv_transform(&self, n: i64) -> i64 {
        n - self.ranges.iter().find(|(r, _)| r.contains(&n)).map(|(_, c)| *c).unwrap_or(0)
    }
}

impl Day<Info> for Day5 {
    fn parse_file(&self, file_content: String) -> Info {
        let nlnl = if file_content.find("\r").is_some() { "\r\n\r\n" } else { "\n\n" };
        let mut split = file_content.split(nlnl);
        
        let seeds = split.next().unwrap()[7..].split(" ").map(str::parse).map(Result::unwrap).collect();
        
        let maps = split.map(|x| {
            Map {
                ranges: x.lines().skip(1)
                    .map(|x| x.split(" "))
                    .map(|mut x| {
                        (x.next().unwrap().parse::<i64>().unwrap(), 
                         x.next().unwrap().parse::<i64>().unwrap(), 
                         x.next().unwrap().parse::<i64>().unwrap())
                    })
                    .map(|(d, s, l)| (s..s+l, d - s))
                    .collect()
            }
            
        }).collect();
        
        Info {
            seeds,
            maps
        }
    }

    fn part_1(&self, data: &Info) -> i64 {
        dbg!(&data.maps);
        
        data.seeds.iter()
            .map(|&x| data.transform(x))
            .min().unwrap() as i64
    }

    fn part_2(&self, data: &Info) -> i64 {
        
        fn intersects(a: &Range<i64>, b: &Range<i64>) -> bool {
            a.contains(&b.start) || b.contains(&a.start)
        }
        
        let ranges: Vec<Range<i64>> = data.seeds.iter()
            .chunks(2)
            .into_iter().map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .map(|(s, l)| *s..s+l )
            .collect();
        
        dbg!(&ranges);
        
        // dbg!(intersect((13..18, 12), (7..15, 4)));
        // 
        // let t1 = vec![(4..10, 5), (13..18, 12), (21..22, 10), (24..29, 1)];
        // let t2 = vec![(7..15, 4), (16..17, 8), (20..23, 15), (30..34, 2)];
        // let te = vec![(4..7, 5), (7..10, 9), (10..13, 16), (13..15, 16), (15..16, 12), (16..17, 20), (17..18, 12), (20..21, 15), (21..22, 25), (22..23, 15)];
        // let mut tr = t1.iter().fold(vec![], |mut fr, (a, c)| {
        //     let mut to_review = vec![(a.clone(),*c)];
        //     while let Some((a,c)) = to_review.pop() {
        //         if let Some(r) = t2.iter().find(|(b, c)| intersects(&a, b)) {
        //             let (more, done) = intersect((a.clone(),c), r.clone());
        //             to_review.extend(more.into_iter());
        //             fr.extend(done.into_iter());
        //         } else {
        //             fr.push((a,c));
        //         }
        //     }
        //     fr
        // });
        // 
        // 
        // tr.sort_by_key(|(r, _)| r.start);
        // 
        // dbg!(tr);
        // let big_map = Map { ranges: data.maps.iter().map(|x| &x.ranges).fold(vec![], |mut tr, t2| {
        //     tr.extend(t2.iter().cloned());
        //     while let Some(Some((a, b))) = tr.iter()
        //         .filter_map(|(ar, ac)| {let ao = offset(ar, *ac);
        //             tr.iter().find(|(br, bc)| ar != br && intersects(&ao, br)})
        //             .map(|b| Some(((ar, *ac), b.clone()))))
        //         .next() {
        //         
        //         // remove a and b
        //         tr.swap_remove(tr.iter().find_position(|x| x == &&a).unwrap().0);
        //         tr.swap_remove(tr.iter().find_position(|x| x == &&b).unwrap().0);
        //         
        //         let ofs = a.1;
        //         
        //         let (ao, bo) = intersect_2((offset(&a.0, a.1), a.1), b);
        //         match ao {
        //             Multi::None => {}
        //             Multi::One(a) => {tr.push((offset(&a.0, -ofs), a.1));}
        //             Multi::Two(a, b) => {
        //                 tr.push((offset(&a.0, -ofs), a.1));
        //                 tr.push((offset(&b.0, -ofs), b.1));
        //             }
        //             Multi::Three(a, b, c) => {
        //                 tr.push((offset(&a.0, -ofs), a.1));
        //                 tr.push((offset(&b.0, -ofs), b.1));
        //                 tr.push((offset(&c.0, -ofs), c.1));
        //             }
        //         };
        //         match bo {
        //             Multi::None => {}
        //             Multi::One(a) => {tr.push(a);}
        //             Multi::Two(a, b) => {
        //                 tr.push(a); tr.push(b);
        //             }
        //             Multi::Three(a, b, c) => {
        //                 tr.push(a); tr.push(b); tr.push(c);
        //             }
        //         }
        //     }
        //     tr
        // })};

        dbg!(data.transform(82));

        let y = data.maps.iter().fold(ranges, |ranges, map| {
            let r = ranges.into_iter().flat_map(|a| {
                let mut to_review = vec![a];
                let mut final_ranges = vec![];
                while let Some(a) = to_review.pop() {
                    if let Some(b) = map.ranges.iter().find(|(b, _)| intersects(&a, b)) {
                        let (difference, intersect) = difference_intersect(a, b.clone());

                        difference.push_to(&mut to_review);
                        final_ranges.push(intersect);
                    } else {
                        final_ranges.push(a);
                    }
                }
                final_ranges
            }).dedup().collect();
            dbg!(r)
        }).iter().map(|r| r.start).min().unwrap();
        dbg!(y);
        y
        
        // let big_map = Map { ranges: data.maps.iter().map(|x| &x.ranges).fold(vec![], |t1, t2| {
        //     let mut tr = t1.iter().fold(vec![], |mut fr, (a, c): &(Range<i64>, i64)| {
        //         let mut tr = vec![(a.start+c..a.end+c, *c)];
        //         while let Some((a,c)) = tr.pop() {
        //             let ao = offset(&a, c);
        //             if let Some(r) = t2.iter().find(|(b, c)| intersects(&ao, b)) {
        //                 let (more, done) = intersect_3((ao, c), r.clone());
        //                 match more {
        //                     Multi::None => {}
        //                     Multi::One(a) => {tr.push((offset(&a.0, -c), a.1));}
        //                     Multi::Two(a, b) => {
        //                         tr.push((offset(&a.0, -c), a.1));
        //                         tr.push((offset(&b.0, -c), b.1));
        //                     }
        //                     Multi::Three(a1, a2, a3) => {
        //                         tr.push((offset(&a1.0, -c), a1.1));
        //                         tr.push((offset(&a2.0, -c), a2.1));
        //                         tr.push((offset(&a3.0, -c), a3.1));
        //                     }
        //                 };
        //                 match done {
        //                     Multi::None => {}
        //                     Multi::One(a) => {fr.push(a);}
        //                     Multi::Two(a, b) => {
        //                         fr.push(a); fr.push(b);
        //                     }
        //                     Multi::Three(a, b, c) => {
        //                         fr.push(a); fr.push(b); fr.push(c);
        //                     }
        //                 };
        //                 
        //             } else {
        //                 fr.push((a,c));
        //             }
        //         }
        //         fr
        //     });
        //     tr.extend(t2.iter().filter(|(a, c)| !t1.iter().any(|(b,_)| intersects(&(a.start+c..a.end+c), b))).cloned());
        //     
        //     dbg!(tr.len());
        //     
        //     tr
        //     // dbg!(tr)
        //     // t1 = tr
        // })};
        // 
        // dbg!(&big_map);
        // dbg!(big_map.transform(13));
        
        // let big_map = Map { ranges: data.maps.iter().map(|x| &x.ranges).fold(vec![], |t1, t2| {
        //     let mut tr = t1.iter().fold(vec![], |mut fr, (a, c): &(Range<i64>, i64)| {
        //         let mut to_review = vec![(a.start+c..a.end+c,*c)];
        //         while let Some((a,c)) = to_review.pop() {
        //             if let Some(r) = t2.iter().find(|(b, c)| intersects(&a, b)) {
        //                 let (more, done) = intersect((a.clone(),c), r.clone());
        //                 to_review.extend(more.into_iter().map(|(r2, c2)| (r2.start-c..r2.end-c, c2)));
        //                 fr.extend(done.into_iter());
        //             } else {
        //                 fr.push((a,c));
        //             }
        //         }
        //         fr
        //     });
        //     tr.extend(t2.iter().filter(|(a, c)| !t1.iter().any(|(b,_)| intersects(&(a.start+c..a.end+c), b))).cloned());
        //     
        //     dbg!(tr)
        //     // t1 = tr
        // })};
        
        // big_map.inv_transform()
        // dbg!(big_map.transform(13));
        // data.maps.iter().map(|x| x.ranges)
        
        // 46
        // 0
    }
}

pub fn offset(a: &Range<i64>, d: i64) -> Range<i64> { (a.start+d)..(a.end+d) }

#[test]
fn test_offset() {
    assert_eq!(offset(&(0..10), 10), 10..20);
}

pub fn intersect(a: (Range<i64>, i64), b: (Range<i64>, i64)) 
    -> (Vec<(Range<i64>, i64)>, Vec<(Range<i64>, i64)>) {
    // Just debug function, ignore
    fn fmt(o: &Ordering) -> char {
        match o {
            Ordering::Less => '<',
            Ordering::Equal => '=',
            Ordering::Greater => '>'
        }
    }

    match (a.0.start.cmp(&b.0.start), b.0.start.cmp(&a.0.end), a.0.end.cmp(&b.0.end)) {
        // ::
        (Ordering::Equal, Ordering::Less, Ordering::Equal) => (vec![], vec![(a.0, a.1 + b.1)]),
        // :.'
        (Ordering::Equal, Ordering::Less, Ordering::Greater) => (vec![(b.0.end..a.0.end, a.1)], vec![(b.0, a.1 + b.1)]),
        // :'.
        (Ordering::Equal, Ordering::Less, Ordering::Less) => (vec![(a.0.end..b.0.end, a.1)], vec![(a.0, a.1 + b.1)]),
        // .':
        (Ordering::Greater, Ordering::Less, Ordering::Equal) => (vec![], vec![(b.0.start..a.0.start, b.1), (a.0, a.1 + b.1)]),
        // '.:
        (Ordering::Less, Ordering::Less, Ordering::Equal) => (vec![(a.0.start..b.0.start, a.1)], vec![(b.0, a.1 + b.1)]),
        // .'.'
        (Ordering::Greater, Ordering::Less, Ordering::Greater) => (vec![(b.0.end..a.0.end,a.1)], vec![(b.0.start..a.0.start, b.1), (a.0.start..b.0.end, a.1 + b.1)]),
        // ''..
        (Ordering::Less, Ordering::Greater, Ordering::Less) => (vec![a], vec![b]),
        // ':.
        (Ordering::Less, Ordering::Equal, Ordering::Less) => (vec![a], vec![b]),
        // .''.
        (Ordering::Greater, Ordering::Less, Ordering::Less) => (vec![], vec![(b.0.start..a.0.start, b.1), (a.0.clone(), a.1 + b.1), (a.0.end..b.0.end, b.1)]),
        // '..'
        (Ordering::Less, Ordering::Less, Ordering::Greater) => (vec![(a.0.start..b.0.start, a.1), (b.0.end..a.0.end, a.1)], vec![(b.0, a.1 + b.1)]),
        // '.'.
        (Ordering::Less, Ordering::Less, Ordering::Less) => (vec![(a.0.start..b.0.start, a.1)], vec![(b.0.start..a.0.end, a.1 + b.1), (a.0.end..b.0.end, b.1)]),
        (o1, o2, o3) =>
            panic!("Error: Unused combination {} {} {} for {}..{} and {}..{}", fmt(&o1), fmt(&o2), fmt(&o3), a.0.start, a.0.end, b.0.start, b.0.end)
    }
}

fn difference_intersect(a: Range<i64>, b: (Range<i64>, i64))
                        -> (Multi<Range<i64>>, Range<i64>) {
    // Just debug function, ignore
    fn fmt(o: &Ordering) -> char {
        match o {
            Ordering::Less => '<',
            Ordering::Equal => '=',
            Ordering::Greater => '>'
        }
    }

    match (a.start.cmp(&b.0.start), b.0.start.cmp(&a.end), a.end.cmp(&b.0.end)) {
        // ::
        (Ordering::Equal, Ordering::Less, Ordering::Equal) => (Multi::None, offset(&a, b.1)),
        // :.'
        (Ordering::Equal, Ordering::Less, Ordering::Greater) => (Multi::One(b.0.end..a.end), offset(&b.0, b.1)),
        // :'.
        (Ordering::Equal, Ordering::Less, Ordering::Less) => (Multi::None, offset(&a, b.1)),
        // .':
        (Ordering::Greater, Ordering::Less, Ordering::Equal) => (Multi::None, offset(&a, b.1)),
        // '.:
        (Ordering::Less, Ordering::Less, Ordering::Equal) => (Multi::One(a.start..b.0.start), offset(&b.0, b.1)),
        // .'.'
        (Ordering::Greater, Ordering::Less, Ordering::Greater) =>
            (Multi::One(b.0.end..a.end), offset(&(a.start..b.0.end), b.1)),
        // ''..
        // (Ordering::Less, Ordering::Greater, Ordering::Less) => (vec![a], vec![b]),
        // ':.
        // (Ordering::Less, Ordering::Equal, Ordering::Less) => (vec![a], vec![b]),
        // .''.
        (Ordering::Greater, Ordering::Less, Ordering::Less) => (Multi::None, offset(&(a.end..b.0.end), b.1)),
        // '..'
        (Ordering::Less, Ordering::Less, Ordering::Greater) => (Multi::Two(a.start..b.0.start, b.0.end..a.end), offset(&b.0, b.1)),
        // '.'.
        (Ordering::Less, Ordering::Less, Ordering::Less) => (Multi::One(a.start..b.0.start), offset(&(b.0.start..a.end), b.1)),
        (o1, o2, o3) =>
            panic!("Error: Unused combination {} {} {} for {}..{} and {}..{}", fmt(&o1), fmt(&o2), fmt(&o3), a.start, a.end, b.0.start, b.0.end)
    }
}

// fn intersect_2(a: (Range<i64>, i64), b: (Range<i64>, i64))
//                -> (Multi<(Range<i64>, i64)>, Multi<(Range<i64>, i64)>) {
//     // Just debug function, ignore
//     fn fmt(o: &Ordering) -> char {
//         match o {
//             Ordering::Less => '<',
//             Ordering::Equal => '=',
//             Ordering::Greater => '>'
//         }
//     }
//
//     match (a.0.start.cmp(&b.0.start), b.0.start.cmp(&a.0.end), a.0.end.cmp(&b.0.end)) {
//         // ::
//         (Ordering::Equal, Ordering::Less, Ordering::Equal) => (Multi::None, Multi::One((a.0, a.1 + b.1))),
//         // :.'
//         (Ordering::Equal, Ordering::Less, Ordering::Greater) => (Multi::Two((b.0.end..a.0.end, a.1), (b.0, a.1 + b.1)), Multi::None),
//         // :'.
//         (Ordering::Equal, Ordering::Less, Ordering::Less) => (Multi::One((a.0.clone(), a.1 + b.1)), Multi::One((a.0.end..b.0.end, b.1))),
//         // .':
//         (Ordering::Greater, Ordering::Less, Ordering::Equal) => (Multi::One((a.0.clone(), a.1 + b.1)), Multi::One((b.0.start..a.0.start, b.1))),
//         // '.:
//         (Ordering::Less, Ordering::Less, Ordering::Equal) => (Multi::Two((a.0.start..b.0.start, a.1), (b.0, a.1 + b.1)), Multi::None),
//         // .'.'
//         (Ordering::Greater, Ordering::Less, Ordering::Greater) => (Multi::Two((a.0.start..b.0.end, a.1 + b.1), (b.0.end..a.0.end, a.1)), Multi::One((b.0.start..a.0.start, b.1))),
//         // ''..
//         // (Ordering::Less, Ordering::Greater, Ordering::Less) => (vec![a], vec![b]),
//         // ':.
//         // (Ordering::Less, Ordering::Equal, Ordering::Less) => (vec![a], vec![b]),
//         // .''.
//         (Ordering::Greater, Ordering::Less, Ordering::Less) => (Multi::One((a.0.clone(),a.1+b.1)), Multi::Two((b.0.start..a.0.end, b.1), (a.0.end..b.0.start, b.1))),
//         // '..'
//         (Ordering::Less, Ordering::Less, Ordering::Greater) => (Multi::Three((a.0.start..b.0.end, a.1), (b.0.clone(), a.1 + b.1), (b.0.end..a.0.end, a.1)), Multi::None),
//         // '.'.
//         (Ordering::Less, Ordering::Less, Ordering::Less) => (Multi::Two((a.0.start..b.0.start, a.1), (b.0.start..a.0.end,a.1+b.1)), Multi::One((a.0.end..b.0.end, b.1))),
//         (o1, o2, o3) =>
//             panic!("Error: Unused combination {} {} {} for {}..{} and {}..{}", fmt(&o1), fmt(&o2), fmt(&o3), a.0.start, a.0.end, b.0.start, b.0.end)
//     }
// }

#[derive(Debug)]
enum Multi<T> {
    None,
    One(T),
    Two(T, T),
}

impl<T> Multi<T> {
    pub fn push_to(self, vec: &mut Vec<T>) {
        match self {
            Multi::None => {}
            Multi::One(a) => {vec.push(a)}
            Multi::Two(a, b) => {vec.push(a); vec.push(b);}
        }
    }
}