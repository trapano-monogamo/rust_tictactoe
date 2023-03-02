use std::io::*;

enum EndTurn {
    Win(char),
    Draw,
    Continue,
}

#[derive(PartialEq, Copy, Clone)]
enum BoardItem {
    Empty,
    Full(char),
}

const MARKERS: [char; 2] = ['O', 'X'];

const COMBINATIONS: [[usize; 3]; 8] = [
	[0,1,2],  [0,3,6],  [0,4,8],
	[3,4,5],  [1,4,7],  [2,4,6],
	[6,7,8],  [2,5,8],
];

struct GameState {
    board: [BoardItem; 9],
}

impl GameState {
    pub fn new() -> GameState {
        GameState { board: [BoardItem::Empty; 9] }
    }

    fn user_input(prompt: String) -> usize {
        let mut input = String::from("");
        print!("{prompt}");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        return match (&input[0..input.len()-1]).parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid input...");
                GameState::user_input(prompt)
            },
        };
    }

    fn draw(&self) {
        println!("-------");
        for i in 0..3 {
            print!("|");
            for j in 0..3 {
                match self.board[i * 3 + j] {
                    BoardItem::Empty => print!(" |"),
                    BoardItem::Full(m) => print!("{m}|"),
                };
            }
            print!("\n-------\n");
        }
    }

    fn check_win(&self) -> EndTurn {
        for c in COMBINATIONS {
            if self.board[c[0]] == self.board[c[1]] && self.board[c[1]] == self.board[c[2]] &&
                self.board[c[0]] != BoardItem::Empty && self.board[c[1]] != BoardItem::Empty && self.board[c[2]] != BoardItem::Empty {
                    return EndTurn::Win(match self.board[c[0]] { BoardItem::Full(m) => m, _ => panic!() });
            }
        }
        if self.board.iter().fold(true, |acc, x| acc && (*x != BoardItem::Empty)) {
            EndTurn::Draw
        } else { EndTurn::Continue }
    }

    fn validate_inpu(&self, n: usize) -> bool {
        if n < 9 && self.board[n] == BoardItem::Empty { true } else { false }
    }

    pub fn game_loop(&mut self) {
        let mut turn: usize = 0;
        let mut marker: char;
        loop {
            println!("Taking turns, type a number between 0 and 8 (in order from topleft to bottom right)");
            self.board = [BoardItem::Empty; 9];
            loop {
                self.draw();

                turn += 1;
                marker = MARKERS[turn % 2];

                loop {
                    let input = GameState::user_input(format!("[{marker}]: "));
                    if self.validate_inpu(input) {
                        self.board[input] = BoardItem::Full(marker);
                        break;
                    } else {
                        println!("Invalid input...");
                        continue;
                    }
                }

                match self.check_win() {
                    EndTurn::Win(m) => {
                        self.draw();
                        println!("'{m}' wins!");
                        break;
                    },
                    EndTurn::Draw => {
                        self.draw();
                        println!("It's a draw!");
                        break;
                    },
                    EndTurn::Continue => continue,
                };
            }
            let mut play_again = String::from("");
            print!("Play again? [y/n] ");
            stdout().flush().unwrap();
            stdin().read_line(&mut play_again).unwrap();
            if play_again.contains("n") { break; }
        }
        println!("Thanks for playing! <3");
    }
}

fn main() { GameState::new().game_loop(); }
