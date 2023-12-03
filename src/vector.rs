use std::ops::Add;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct V2I {
    pub x: i32,
    pub y: i32
}

impl V2I {
    pub fn new(x: i32, y: i32) -> V2I {
        V2I {x, y}
    }
}

impl Add for V2I {
    type Output = V2I;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}