use std::collections::VecDeque;

use rand::Rng;

fn main() {
    let mut board = BoardData::new();
    board.print_board();
    board.move_right();

    println!("============");
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
            let mut empty_spots: VecDeque<usize> = VecDeque::new();
            let mut last_block_index: Option<usize> = Some(3);

            for x in (0..4).rev() {
                if self.board[x][y] == 0 {
                    empty_spots.push_back(x);
                    continue;
                }

                if let Some(last_x) = last_block_index {
                    if self.board[x][y] == self.board[last_x][y] && x != last_x {
                        self.board[last_x][y] *= 2;
                        self.board[x][y] = 0;

                        last_block_index = None;
                        continue;
                    }
                }
                last_block_index = Some(x);

                if let Some(empty_x) = empty_spots.pop_front() {
                    self.board[empty_x][y] = self.board[x][y];
                    self.board[x][y] = 0;

                    empty_spots.push_front(x);
                }
            }
        }
    }
}
