pub mod board;
pub mod search;
use std::time::Instant;

use board::Board;
use search::Search;

#[rustfmt::skip]
pub static BOARD: &str = 
r#"O  1   XOOXO
---X1   O     1
--- O  X   1 11
--- O1O11X     
---  X 11   OOX
--- O1OO 1  X 1
---1 X  X O1XX 
---OXX   O1 O  
---     11XOXO 
---XO X   O  O 
---X     X   X1
---OXXOX   X  O
"#
;

fn parse_board(board: Option<Board>) -> Board {
    match board {
        Some(b) => b,
        None => {
            println!("Failed to parse board");
            panic!();
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut board: Board;

    if args.len() > 1 {
        let input = args.get(1).unwrap();
        board = parse_board(Board::new(input));
    } else {
        board = parse_board(Board::new(BOARD));
    }

    board.print_board();

    let start = Instant::now();
    let mut search = Search::new(&mut board);
    let succes = search.search_board(0);
    let end = start.elapsed();

    if !succes {
        println!("Failed to solve binairo! Please check input");
    }

    println!("Solution");
    println!("Solved in {}ms", end.as_secs_f32() * 1000.);
    board.print_board();
}
