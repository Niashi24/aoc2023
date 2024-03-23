use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::backtracking::backtracking::{Action, backtracking};

#[derive(Debug)]
pub struct Board {
    board: [[SudokuCell; 9]; 9],
    index: usize,
}

static mut states_explored: i32 = 0;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y,by) in self.board.iter().enumerate() {
            for (x, bx) in by.iter().enumerate() {
                if y * 9 + x == self.index {
                    write!(f,"[{}", bx)?;
                } else {
                    write!(f," {}", bx)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SudokuCell {
    Clue(u8),
    Guess(u8),
    None,
}

impl Display for SudokuCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuCell::Clue(x) => write!(f, "{}", x.to_string().red()),
            SudokuCell::Guess(x) => write!(f, "{}", x.to_string().blue()),
            SudokuCell::None => write!(f," ")
        }
    }
}

impl SudokuCell {
    pub fn get_value(&self) -> Option<u8> {
        match self {
            SudokuCell::Clue(x) => Some(*x),
            SudokuCell::Guess(x) => Some(*x),
            SudokuCell::None => None
        }
    }
}

impl Board {
    pub fn is_valid_action(&self, action: &AddNum) -> bool {
        let (x, y) = (self.index % 9, self.index / 9);
        // Check rows
        let mut filled = [false; 9];
        filled[action.0 as usize - 1] = true;
        for i in (0..9).filter_map(|x| self.get_xy(x, y).get_value()) {
            let i = i as usize - 1;
            if filled[i] {
                return false;
            }
            filled[i] = true;
        }

        let mut filled = [false; 9];
        filled[action.0 as usize - 1] = true;
        for i in (0..9).filter_map(|y| self.get_xy(x, y).get_value()) {
            let i = i as usize - 1;
            if filled[i] {
                return false;
            }
            filled[i] = true;
        }

        let (box_x, box_y) = (self.index / 3 % 3, self.index / 27);
        let mut filled = [false; 9];
        filled[action.0 as usize - 1] = true;
        for i in self.box_idx_iter(box_x, box_y).into_iter()
            .map(|i| self.get_idx(i))
            .filter_map(|c| c.get_value()) {
            let i = i as usize - 1;
            if filled[i] {
                return false;
            }
            filled[i] = true;
        }

        true
    }

    fn get_idx(&self, idx: usize) -> &SudokuCell {
        self.get_xy(idx % 9, idx / 9)
    }

    fn get_xy(&self, x: usize, y: usize) -> &SudokuCell {
        &self.board[y][x]
    }

    fn box_idx_iter(&self, box_x: usize, box_y: usize) -> impl IntoIterator<Item=usize> {
        match box_x {
            0 => {
                match box_y {
                    0 => [0, 1, 2, 9, 10, 11, 18, 19, 20],
                    1 => [27, 28, 29, 36, 37, 38, 45, 46, 47],
                    2 => [54, 55, 56, 63, 64, 65, 72, 73, 74],
                    _ => [0, 0, 0, 0, 0, 0, 0, 0, 0]
                }
            }
            1 => {
                match box_y {
                    0 => [3, 4, 5, 12, 13, 14, 21, 22, 23],
                    1 => [30, 31, 32, 39, 40, 41, 48, 49, 50],
                    2 => [57, 58, 59, 66, 67, 68, 75, 76, 77],
                    _ => [0, 0, 0, 0, 0, 0, 0, 0, 0]
                }
            }
            2 => {
                match box_y {
                    0 => [6, 7, 8, 15, 16, 17, 24, 25, 26],
                    1 => [33, 34, 35, 42, 43, 44, 51, 52, 53],
                    2 => [60, 61, 62, 69, 70, 71, 78, 79, 80],
                    _ => [0, 0, 0, 0, 0, 0, 0, 0, 0]
                }
            }
            _ => [0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
    }

    pub fn add(&mut self, val: u8) {
        self.board[self.index / 9][self.index % 9] = SudokuCell::Guess(val);
        self.index += 1;
        // println!("{}", self);
        while self.index < 81 && matches!(self.get_idx(self.index), SudokuCell::Clue(_)) {
            self.index += 1;
            // println!("{}", self);
        }
    }

    pub fn pop(&mut self) {
        if self.index == 0 { return; }
        
        unsafe { states_explored += 1; }

        self.board[self.index / 9][self.index % 9] = SudokuCell::None;
        self.index -= 1;
        while self.index > 0 && matches!(self.get_idx(self.index), SudokuCell::Clue(_)) {
            self.index -= 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index >= 81
    }
}

#[derive(Copy, Clone)]
pub struct AddNum(u8);

const ACTIONS: [AddNum; 9] = [
    AddNum(1),
    AddNum(2),
    AddNum(3),
    AddNum(4),
    AddNum(5),
    AddNum(6),
    AddNum(7),
    AddNum(8),
    AddNum(9),
];

impl Action<Board> for AddNum {
    fn execute(&self, mut state: Board) -> Board {
        state.add(self.0);
        state
    }

    fn undo(&self, mut state: Board) -> Board {
        state.pop();
        state
    }
}

pub fn test() {
    let board = [
        [5,3,0,0,7,0,0,0,0],
        [6,0,0,1,9,5,0,0,0],
        [0,9,8,0,0,0,0,6,0],
        [8,0,0,0,6,0,0,0,3],
        [4,0,0,8,0,3,0,0,1],
        [7,0,0,0,2,0,0,0,6],
        [0,6,0,0,0,0,2,8,0],
        [0,0,0,4,1,9,0,0,5],
        [0,0,0,0,8,0,0,7,9]
    ];
    let board = [
        [1,9,0,0,4,0,0,0,0],
        [0,0,0,0,8,6,0,4,7],
        [6,0,0,0,0,1,5,0,3],
        [7,0,4,1,6,0,3,8,2],
        [9,3,6,5,2,0,0,1,4],
        [0,8,0,7,3,0,0,0,0],
        [0,0,0,0,0,3,0,5,0],
        [4,0,5,0,1,0,0,0,0],
        [0,1,9,0,0,0,4,0,8]
    ];
    
    let board = [
        [5,7,0,4,0,1,0,6,0],
        [0,0,1,0,9,0,0,0,0],
        [0,0,0,0,8,0,4,0,0],
        [6,0,0,0,0,0,0,0,2],
        [0,0,5,3,0,7,9,0,0],
        [0,0,0,0,1,0,0,0,0],
        [0,0,7,5,0,4,3,0,0],
        [0,0,0,8,0,0,0,0,0],
        [0,3,0,0,0,0,0,9,0]
    ];
    
    let s_index = board.iter().enumerate()
        .filter_map(|(y, b)| b.iter().enumerate()
            .find(|(x, c)| **c == 0)).next().unwrap().0;
    
    let board = Board {
        index: s_index,
        board: board.map(|y| y.map(|i| match i {
            0 => SudokuCell::None,
            x => SudokuCell::Clue(x)
        })),
    };
    
    let x = backtracking(board, |_| ACTIONS.clone(), Board::is_finished, Board::is_valid_action);
    if let Some(b) = x {
        println!("{}", b);
    } else {
        println!("Solution not found");
    }
    
    println!("States explored: {}", unsafe { states_explored });
}

