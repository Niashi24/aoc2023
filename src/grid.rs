use std::fmt::{Display, Formatter};
use num::traits::Euclid;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    pub w: usize,
    pub h: usize,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let h = grid.len();
        let w = grid.get(0).unwrap_or(&vec![]).len();
        Self {
            grid,
            w,
            h
        }
    }
    
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).and_then(|y| y.get(x))
    }
    
    pub fn get_i(&self, x: i64, y: i64) -> Option<&T> {
        if x < 0 || y < 0 { None }
        else { self.get(x as usize, y as usize) }
    }
    
    pub fn get_cycle(&self, mut x: i64, mut y: i64) -> Option<&T> {
        x = x.rem_euclid(self.w as i64);
        y = y.rem_euclid(self.h as i64);
        self.get(x as usize, y as usize)
    }
    
    pub fn positions<FN: Fn(&T) -> bool>(&self, predicate: FN) -> Vec<(usize, usize)> {
        let mut pos = vec![];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if predicate(item) {
                    pos.push((x, y));
                }
            }
        }
        pos
    }
    
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.get(self.x, self.y)
            .map(|x| ((self.x, self.y), x));
        
        self.x += 1;
        if self.x == self.grid.w {
            self.y += 1;
            self.x = 0;
            if self.y == self.grid.h {
                return None;
            }
        }
        
        item
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for col in row.iter() {
                write!(f, "{}", col)?;
            }
            writeln!(f, "")?;
        }
        
        Ok(())
    }
}

