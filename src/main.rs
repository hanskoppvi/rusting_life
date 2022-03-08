extern crate termion;

use std::thread;
use std::time::Duration;
use termion::color;

fn print_board(board: &mut [[i32; 8]; 8]) {
    print!("\x1B[2J\x1B[1;1H");
    for y in 0..8 {
        for x in 0..8 {
            if board[y][x] == 1 {
                print!("{}  {}", color::Bg(color::White), color::Bg(color::Reset));
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}

fn check_srd_cells(o_x: &i32, o_y: &i32, board: &mut [[i32; 8]; 8]) -> i32 {
    let mut count: i32 = 0i32;
    for i in 0..8 {
        let mut y = *o_y;
        let mut x = *o_x;
        match i {
            0 => {
                x -= 1;
                y -= 1;
            }
            1 => {
                y -= 1;
            }
            2 => {
                x += 1;
                y -= 1;
            }
            3 => {
                x -= 1;
            }
            4 => {
                x += 1;
            }
            5 => {
                x -= 1;
                y += 1;
            }
            6 => {
                y += 1;
            }
            7 => {
                x += 1;
                y += 1;
            }
            _ => println!("-"),
        }
        if y < 0 {
            y = 7;
        }
        if x < 0 {
            x = 7;
        }
        if y > 7 {
            y = 0;
        }
        if x > 7 {
            x = 0;
        }
        let index_y = y as usize;
        let index_x = x as usize;
        count += board[index_y][index_x];
    }
    return count;
}

fn next_gen(board: &mut [[i32; 8]; 8]) {
    let mut cells = [[1; 8]; 8];
    for y in 0..8 {
        for x in 0..8 {
            let alive = board[y][x];
            let o_x = x as i32;
            let o_y = y as i32;
            let num_srd_cells = check_srd_cells(&o_x, &o_y, board);
            if alive == 1 {
                cells[y][x] = if (num_srd_cells <= 1) | (num_srd_cells >= 4) {
                    0
                } else {
                    1
                };
            } else {
                cells[y][x] = if num_srd_cells == 3 { 1 } else { 0 };
            }
        }
    }
    for y in 0..8 {
        for x in 0..8 {
            board[y][x] = cells[y][x];
        }
    }
    println!("");
}

fn init_board(board: &mut [[i32; 8]; 8]) {
    board[1][3] = 1;
    board[2][1] = 1;
    board[2][3] = 1;
    board[3][2] = 1;
    board[3][3] = 1;
}

fn main() {
    let cells: &mut [[i32; 8]; 8] = &mut [[0; 8]; 8];
    init_board(cells);
    let five_h_ms = Duration::from_millis(100);
    loop {
        print_board(cells);
        thread::sleep(five_h_ms);
        next_gen(cells);
    }
}
