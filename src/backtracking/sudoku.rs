use std::fmt::{Display, Formatter};
use std::time::Instant;
use colored::Colorize;
use crate::backtracking::backtracking::{Action, backtracking, backtracking_iterative};

#[derive(Debug)]
pub struct Board {
    board: [[SudokuCell; 9]; 9],
    index: usize,
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
        [0,1,2,9,10,11,18,19,20]
            .map(|x| x + box_x * 3 + box_y * 27)
    }

    pub fn add(&mut self, val: u8) {
        self.board[self.index / 9][self.index % 9] = SudokuCell::Guess(val);

        unsafe { states_explored += 1; }
        
        self.index += 1;
        while self.index < 81 && matches!(self.get_idx(self.index), SudokuCell::Clue(_)) {
            self.index += 1;
        }
    }

    pub fn pop(&mut self) {
        if self.index == 0 { return; }

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

static mut states_explored: i32 = 0;

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for by in self.board.iter() {
            for bx in by {
                write!(f,"{}  ", bx)?;
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

    fn is_valid(&self, state: &Board) -> bool {
        state.is_valid_action(self)
    }
}

pub fn test() {
    
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
    
    let board = [
        [8,0,0,0,0,0,0,0,0],
        [0,0,3,6,0,0,0,0,0],
        [0,7,0,0,9,0,2,0,0],
        [0,5,0,0,0,7,0,0,0],
        [0,0,0,0,4,5,7,0,0],
        [0,0,0,1,0,0,0,3,0],
        [0,0,1,0,0,0,0,6,8],
        [0,0,8,5,0,0,0,1,0],
        [0,9,0,0,0,0,4,0,0]
    ];
    
    // let board = [
    //     [1,0,0,0,0,7,0,9,0],
    //     [0,3,0,0,2,0,0,0,8],
    //     [0,0,9,6,0,0,5,0,0],
    //     [0,0,5,3,0,0,9,0,0],
    //     [0,1,0,0,8,0,0,0,2],
    //     [6,0,0,0,0,4,0,0,0],
    //     [3,0,0,0,0,0,0,1,0],
    //     [0,4,0,0,0,0,0,0,7],
    //     [0,0,7,0,0,0,3,0,0]
    // ];
    
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
    
    let now = Instant::now();
    
    let x = backtracking_iterative(board, |_| ACTIONS, Board::is_finished);
    println!("Time elapsed: {:.2?}", now.elapsed());
    if let Some(b) = x {
        println!("{}", b);
    } else {
        println!("Solution not found");
    }
    
    println!("States explored: {}", unsafe { states_explored });
}

