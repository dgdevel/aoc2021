
use super::aocutil;
use regex::*;

#[derive(Debug)]
struct Row {
    nums : [u8; 5]
}

#[derive(Debug)]
struct Board {
    rows : [Row; 5],
    marks : [bool;25]
}

trait BingoBoard {
    fn mark(&mut self, row:u8, col:u8);
    fn search(&mut self, num:u8) -> Option<[u8;2]>;
    fn bingo(&self) -> bool;
    fn sum_of_unmarked(&self) -> u32;
}

impl BingoBoard for Board {
    fn mark(&mut self, row:u8, col:u8) {
        self.marks[(row * 5 + col) as usize] = true;
    }
    fn search(&mut self, num:u8) -> Option<[u8;2]> {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].nums.len() {
                if self.rows[i].nums[j] == num {
                    return Some([i as u8,j as u8])
                }
            }
        }
        None
    }
    fn bingo(&self) -> bool {
        for n in 0..5 {
            if self.marks[n*5] && self.marks[1+(n*5)] && self.marks[2+(n*5)] && self.marks[3+(n*5)] && self.marks[4+(n*5)] {
                return true
            }
        }
        for n in 0..5 {
            if self.marks[n] && self.marks[5+n] && self.marks[10+n] && self.marks[15+n] && self.marks[20+n] {
                return true
            }
        }
        false
    }
    fn sum_of_unmarked(&self) -> u32 {
        let mut sum : u32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marks[i*5+j] {
                    sum += self.rows[i].nums[j] as u32;
                }
            }
        }
        sum
    }
}

fn parse_board_line(line : &String) -> [u8;5] {
    let num = Regex::new(r"\d+").unwrap();
    let matches = num.find_iter(line).map(|m| m.as_str().parse::<u8>().unwrap()).collect::<Vec<u8>>();
    [ matches[0], matches[1], matches[2], matches[3], matches[4] ]
}

fn parse_board(lines_board : Vec<String>) -> Board {
     Board {
         rows : [
             Row { nums : parse_board_line(&lines_board[0]) },
             Row { nums : parse_board_line(&lines_board[1]) },
             Row { nums : parse_board_line(&lines_board[2]) },
             Row { nums : parse_board_line(&lines_board[3]) },
             Row { nums : parse_board_line(&lines_board[4]) }
         ],
         marks : [false;25]
     }
}

pub fn p4_1() -> String {
    let lines = aocutil::read_file_to_string_list("p4_1".to_string());
    let nums = lines[0].split(",").collect::<Vec<&str>>().iter().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let num_of_boards = (lines.len() - 1) / 6;
    let mut boards : Vec<Board> = Vec::new();
    for n in 0..num_of_boards {
        let lines_board = (&lines[2+n*6..7+n*6]).to_vec();
        let board = parse_board(lines_board);
        boards.push(board);
    }

    for num in nums {
        // println!("num {}", num);
        for b in 0..boards.len() {
            let board = &mut boards[b];
            let res = board.search(num);
            if res.is_some() {
                let pos = res.unwrap();
                board.mark(pos[0], pos[1]);
                if board.bingo() {
                    // println!("bingo {}", b);
                    // println!("{:?}", board);
                    return format!("{:?}", board.sum_of_unmarked() * num as u32);
                }
            }
        }
    }
    unreachable!();
}

pub fn p4_2() -> String {
    let lines = aocutil::read_file_to_string_list("p4_1".to_string());
    let nums = lines[0].split(",").collect::<Vec<&str>>().iter().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let num_of_boards = (lines.len() - 1) / 6;
    let mut boards : Vec<Board> = Vec::new();
    for n in 0..num_of_boards {
        let lines_board = (&lines[2+n*6..7+n*6]).to_vec();
        let board = parse_board(lines_board);
        boards.push(board);
    }
    let mut last_board : i8 = -1;
    let mut last_num : i8 = -1;
    for num in nums {
        // println!("num {}", num);
        for b in 0..boards.len() {
            let board = &mut boards[b];
            if ! board.bingo() {
                let res = board.search(num);
                if res.is_some() {
                    let pos = res.unwrap();
                    board.mark(pos[0], pos[1]);
                    if board.bingo() {
                        // println!("bingo {}", b);
                        last_board = b as i8;
                        last_num = num as i8;
                    }
                }
            }
        }
    }
    // println!("{:?}", last_board);
    // println!("{:?}", boards[last_board as usize]);
    // println!("{:?}", boards[last_board as usize].sum_of_unmarked());
    // println!("{:?}", last_num);
    let score = boards[last_board as usize].sum_of_unmarked() * last_num as u32;
    format!("{:?}", score).to_string()
}

