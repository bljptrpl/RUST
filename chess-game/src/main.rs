extern crate chess;

use chess::{Board, ChessMove, Color, File, Piece, Rank, Square};

fn main() {
    // Create a new chess board with the default setup
    let board = Board::default();

    // Display the initial board
    println!("{}", board);

    // Create a move from e2 to e4
    let from = Square::make_square(Rank::Second, File::E);
    let to = Square::make_square(Rank::Fourth, File::E);
    let chess_move = ChessMove::new(from, to, None);

    // Make the move on the board
    let new_board = board.make_move_new(chess_move);

    // Display the new board
    println!("{}", new_board);
}
