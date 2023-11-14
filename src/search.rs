use crate::board::{Board, OPEN};

pub struct Search<'a> {
    board: &'a mut Board,
}

impl Search<'_> {
    pub fn new(board: &mut Board) -> Search {
        Search { board }
    }

    fn get_empty_square(&self, square: usize) -> usize {
        for i in square..self.board.size {
            if self.board.get_square_value(i) == OPEN {
                return i;
            }
        }
        255
    }

    pub fn search_board(&mut self, square: usize) -> bool {
        let square = self.get_empty_square(square);
        // Board is completed
        if square == 255 {
            return true;
        }

        for i in 0..self.board.options.len() {
            if self.board.valid_square_value(square, i as u8) {
                self.board.set_square_value(square, i as u8);
                if self.search_board(square) {
                    return true;
                }
            }
            self.board.set_square_value(square, OPEN);
        }
        return false;
    }
}
