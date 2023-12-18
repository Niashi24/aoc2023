use std::fmt::{Display, Formatter};

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

