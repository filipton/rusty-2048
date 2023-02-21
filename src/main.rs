use rand::Rng;
use std::{borrow::Borrow, collections::VecDeque};

mod tests;

fn main() {
    let mut board = BoardData { board: [[0; 4]; 4] };
    board.board[0][0] = 2;
    board.board[1][0] = 2;
    board.board[2][0] = 2;
    board.board[3][0] = 2;

    board.print_board();
    board.move_right();
    board.print_board();
}

struct BoardData {
    board: [[u64; 4]; 4],
}

impl BoardData {
    fn new() -> BoardData {
        let mut b_data = BoardData { board: [[0; 4]; 4] };
        b_data.add_random_tile_count(2);

        return b_data;
    }

    fn print_board(&self) {
        for y in 0..4 {
            for x in 0..4 {
                print!("{} ", self.board[x][y]);
            }
            println!("");
        }
        println!("");
    }

    fn add_random_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..4);
        let mut y = rng.gen_range(0..4);
        while self.board[x][y] != 0 {
            x = rng.gen_range(0..4);
            y = rng.gen_range(0..4);
        }

        let tile_value = if rng.gen_range(0..10) < 2 { 4 } else { 2 };
        self.board[x][y] = tile_value;
    }

    fn add_random_tile_count(&mut self, count: usize) {
        for _ in 0..count {
            self.add_random_tile();
        }
    }

    fn move_right(&mut self) {
        for y in 0..4 {
            for x in (0..4).rev() {
                if self.board[x][y] == 0 {
                    continue;
                }

                let mut block_x = x;
                for x2 in (x + 1)..4 {
                    if self.board[x2][y] == 0 {
                        self.board[x2][y] = self.board[block_x][y];
                        self.board[block_x][y] = 0;
                        block_x = x2;

                        continue;
                    }
                }

                for combine_x in (0..block_x).rev() {
                    if self.board[combine_x][y] == 0 {
                        continue;
                    }

                    if self.board[combine_x][y] != self.board[block_x][y] {
                        break;
                    }

                    self.board[combine_x][y] = 0;
                    self.board[block_x][y] *= 2;
                }

                self.print_board();
            }
        }
    }
}
