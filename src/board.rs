use std::collections::HashMap;

use crossterm::style::Color;
use matrix_display::*;
use rand::Rng;

use crate::utils::{print_formatted, write};

pub struct BoardData {
    pub board: [[u64; 4]; 4],
    pub lost: bool,
    pub debug: bool,
}

impl BoardData {
    pub fn new() -> BoardData {
        let mut b_data = BoardData {
            board: [[0; 4]; 4],
            lost: false,
            debug: false,
        };
        b_data.add_random_tile_count(2);

        return b_data;
    }

    pub fn print_board(&self) {
        let format = Format::new(7, 3);
        let color_theme: HashMap<u64, u8> = HashMap::from([
            (0, 247),
            (2, 78),
            (4, 222),
            (8, 220),
            (16, 214),
            (32, 208),
            (64, 202),
            (128, 196),
            (256, 162),
            (512, 160),
            (1024, 126),
            (2048, 90),
            (4096, 88),
            (8192, 54),
            (16384, 53),
            (32768, 52),
        ]);

        let mut board_matrix: Vec<_> = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                board_matrix.push(cell::Cell::new(
                    parse_text(self.board[y][x]),
                    0,
                    *color_theme.get(&self.board[y][x]).unwrap_or(&0u8),
                ))
            }
        }

        let mut data = matrix::Matrix::new(4, board_matrix);
        let display = MatrixDisplay::new(&format, &mut data);
        display.print(&mut std::io::stdout(), &style::BordersStyle::ArcLight);
    }

    pub fn check_empty(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.board[x][y] == 0 {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn add_random_tile(&mut self) {
        if !self.check_empty() {
            self.lost = true;
            return;
        }

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

    pub fn add_random_tile_count(&mut self, count: usize) {
        for _ in 0..count {
            self.add_random_tile();
        }
    }

    pub fn move_right(&mut self) {
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
            }
        }

        if !self.debug {
            self.add_random_tile();
        }
    }

    pub fn move_left(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.board[x][y] == 0 {
                    continue;
                }

                let mut block_x = x;
                for x2 in (0..x).rev() {
                    if self.board[x2][y] == 0 {
                        self.board[x2][y] = self.board[block_x][y];
                        self.board[block_x][y] = 0;
                        block_x = x2;

                        continue;
                    }
                }

                for combine_x in block_x + 1..4 {
                    if self.board[combine_x][y] == 0 {
                        continue;
                    }
                    if self.board[combine_x][y] != self.board[block_x][y] {
                        break;
                    }

                    self.board[combine_x][y] = 0;
                    self.board[block_x][y] *= 2;
                }
            }
        }

        if !self.debug {
            self.add_random_tile();
        }
    }

    pub fn move_up(&mut self) {
        for x in 0..4 {
            for y in 0..4 {
                if self.board[x][y] == 0 {
                    continue;
                }

                let mut block_y = y;
                for y2 in (0..y).rev() {
                    if self.board[x][y2] == 0 {
                        self.board[x][y2] = self.board[x][block_y];
                        self.board[x][block_y] = 0;
                        block_y = y2;

                        continue;
                    }
                }

                for combine_y in block_y + 1..4 {
                    if self.board[x][combine_y] == 0 {
                        continue;
                    }
                    if self.board[x][combine_y] != self.board[x][block_y] {
                        break;
                    }

                    self.board[x][combine_y] = 0;
                    self.board[x][block_y] *= 2;
                }
            }
        }

        if !self.debug {
            self.add_random_tile();
        }
    }

    pub fn move_down(&mut self) {
        for x in 0..4 {
            for y in (0..4).rev() {
                if self.board[x][y] == 0 {
                    continue;
                }

                let mut block_y = y;
                for y2 in (y + 1)..4 {
                    if self.board[x][y2] == 0 {
                        self.board[x][y2] = self.board[x][block_y];
                        self.board[x][block_y] = 0;
                        block_y = y2;

                        continue;
                    }
                }

                for combine_y in (0..block_y).rev() {
                    if self.board[x][combine_y] == 0 {
                        continue;
                    }
                    if self.board[x][combine_y] != self.board[x][block_y] {
                        break;
                    }

                    self.board[x][combine_y] = 0;
                    self.board[x][block_y] *= 2;
                }
            }
        }

        if !self.debug {
            self.add_random_tile();
        }
    }
}

pub fn parse_text(input: u64) -> String {
    if input > 0 {
        return format!("{}", input);
    }

    return format!("");
}
