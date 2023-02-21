use crossterm::style::Color;
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
        println!("{}\r", "=".repeat(33));

        for y in 0..4 {
            print!("\r\n");
            for x in 0..4 {
                print!("|");
                // print!("{: ^6}", self.board[x][y]);
                print_formatted(
                    format!("{: ^7}", self.board[x][y]),
                    get_block_color(self.board[x][y]),
                    Color::Reset,
                )
            }

            println!("|\r\n\r\n{}\r", "=".repeat(33));
        }

        println!("\r");
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

pub fn get_block_color(value: u64) -> Color {
    match value {
        0 => Color::Rgb { r: 0, g: 0, b: 0 },
        2 => Color::Rgb {
            r: 238,
            g: 228,
            b: 218,
        },
        4 => Color::Rgb {
            r: 237,
            g: 224,
            b: 200,
        },
        8 => Color::Rgb {
            r: 242,
            g: 177,
            b: 121,
        },
        16 => Color::Rgb {
            r: 245,
            g: 149,
            b: 99,
        },
        32 => Color::Rgb {
            r: 246,
            g: 124,
            b: 95,
        },
        64 => Color::Rgb {
            r: 246,
            g: 94,
            b: 59,
        },
        128 => Color::Rgb {
            r: 237,
            g: 207,
            b: 114,
        },
        256 => Color::Rgb {
            r: 237,
            g: 204,
            b: 97,
        },
        512 => Color::Rgb {
            r: 237,
            g: 200,
            b: 80,
        },
        1024 => Color::Rgb {
            r: 237,
            g: 197,
            b: 63,
        },
        2048 => Color::Rgb {
            r: 237,
            g: 194,
            b: 46,
        },
        _ => Color::Reset,
    }
}
