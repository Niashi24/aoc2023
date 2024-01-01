use std::cmp::min;
use crate::day::Day;

pub struct Day13;

pub struct Data {
    grids: Vec<Vec<Vec<bool>>>
}

impl Day<Data> for Day13 {
    fn parse_file(&self, file_content: String) -> Data {
        let line_end = if file_content.find("\r").is_some() { "\r\n\r\n" } else { "\n\n" };
        
        Data {
            grids: file_content.split(line_end).map(|s| {
                s.lines().map(|s| {
                    s.chars().map(|x| {
                        match x {
                            '#' => true,
                            '.' => false,
                            x => panic!("{x}")
                        }
                    }).collect()
                }).collect()
            }).collect()
        }
    }

    fn part_1(&self, data: &Data) -> i64 {
        data.grids.iter().map(|grid| {
            let (w, h) = (grid.get(0).unwrap().len(), grid.len());
            if let Some(r) = (1..h)
                .find(|&i| test_1(RowIterator::new(i, w, h), grid)) {
                return (r as i64) * 100;
            }

            let c =(1..w)
                .find(|&i| test_1(ColumnIterator::new(i, w, h), grid))
                .unwrap() as i64;
            c
        }).sum::<i64>()
    }

    fn part_2(&self, data: &Data) -> i64 {
        data.grids.iter().map(|grid| {
            let (w, h) = (grid.get(0).unwrap().len(), grid.len());
            if let Some(r) = (1..h)
                .find(|&i| test_2(RowIterator::new(i, w, h), grid)) {
                return (r as i64) * 100;
            }

            let c =(1..w)
                .find(|&i| test_2(ColumnIterator::new(i, w, h), grid))
                .unwrap() as i64;
            c
        }).sum::<i64>()
    }
}

struct RowIterator {
    x: usize,
    y: usize,
    i: usize,
    w: usize,
    h: usize,
}

impl RowIterator {
    pub fn new(i: usize, w: usize, h: usize) -> Self {
        Self {
            x: 0,
            y: i,
            i,
            w,
            h,
        }
    }
}

impl Iterator for RowIterator {
    type Item = ((usize, usize), (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.y + 1 > 2 * self.i || self.y == self.h {
            return None;
        }
        
        let out = Some(((self.x, self.y), (self.x, (2 * self.i - 1) - self.y)));
        
        self.x += 1;
        if self.x == self.w {
            self.x = 0;
            self.y += 1;
        }

        out
    }
}

struct ColumnIterator {
    x: usize,
    y: usize,
    i: usize,
    w: usize,
    h: usize
}

impl Iterator for ColumnIterator {
    type Item = ((usize, usize), (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.x + 1 > 2 * self.i || self.x == self.w {
            return None;
        }
        
        let out = Some(((self.x, self.y), ((2 * self.i - 1) - self.x, self.y)));
        self.y += 1;
        if self.y == self.h {
            self.y = 0;
            self.x += 1;
        }
        
        out
    }
}

impl ColumnIterator {
    pub fn new(i: usize, w: usize, h: usize) -> Self {
        Self {
            x: i,
            y: 0,
            i,
            w,
            h,
        }
    }
}

fn test_1<IT>(mut it: IT, grid: &Vec<Vec<bool>>) -> bool
where
    IT: Iterator<Item = ((usize, usize), (usize, usize))>
{
    it.all(|((x1, y1), (x2, y2))| {
        grid.get(y1).unwrap().get(x1).unwrap() ==
            grid.get(y2).unwrap().get(x2).unwrap()
    })
}

fn test_2<IT>(it: IT, grid: &Vec<Vec<bool>>) -> bool
    where
        IT: Iterator<Item = ((usize, usize), (usize, usize))>
{
    it.filter(|((x1, y1), (x2, y2))| {
        grid.get(*y1).unwrap().get(*x1).unwrap() !=
            grid.get(*y2).unwrap().get(*x2).unwrap()
    }).take(2) // stop when count > 1
        .count() == 1
}
