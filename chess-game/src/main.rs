

// Piece
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

// Color
enum Color {
    White,
    Black
}
fn welcome(){
    print!("Hello user! Welcome to Chess developed by Daniel V.\n\n
    If you aren't new type [S] to Start.
    If you are new type [R] for Rules.");
    board();
    //scan for input
    //if Rules
    println!("Rook:\n
    - The Rook can move in a straight line vertically and horizontally.\n
    Example Board:\n");
    println!("Knight:
    - A knight can jump over pieces in an L shape.\n
    - 2 moves vertically or horizontally and 1 move to the side.\n
    Example Board:\n
    ");
    println!("Bishop:");
    println!("Queen:");
    println!("King:");
    println!("Pawn:");
    println!("En passant:\n\n
    The conditions for a pawn to capture an enemy pawn en passant are as follows:\n
    \t- The enemy pawn advanced two squares on the previous turn;\n
    \t- The capturing pawn attacks the square that the enemy pawn passed over.
    If these conditions are met, the capturing pawn can move diagonally forward to the square that the enemy pawn passed, 
    capturing the enemy pawn as if it had moved only one square. If the right to capture en passant is not exercised immediately, 
    it is lost. Making the capture is optional, unless there is no other legal move.");
    
    println!("Castling:\n
    Castling is permitted provided all of the following conditions are met:[5]
    \t- Neither the king nor the rook has previously moved.
    \t- There are no pieces between the king and the rook.
    \t- The king is not currently in check.
    \t- The king does not pass through or finish on a square that is attacked by an enemy piece.
    A player may not castle out of, through, or into check.");

    println!("Pawn Promotion:\n");
    println!("Check/Checkmate:\n");


    //if Start
 
}
//Board
fn board(){
    //initial setup
    println!("
    \n\t    A\t\tB\t\tC\t\tD\t\tE\t\tF\t\tG\t\tH\t\t
    \n8\t   Rook\t\tKnight\t\tBishop\t\tQueen\t\tKing\t\tBishop\t\tKnight\t\tRook\t\t
    \n7\t   Pawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\t
    \n6\t
    \n5\t
    \n4\t
    \n3\t
    \n2\t   Pawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\tPawn\t\t
    \n1\t   Rook\t\tKnight\t\tBishop\t\tQueen\t\tKing\t\tBishop\t\tKnight\t\tRook\t\t
    \n\t    A\t\tB\t\tC\t\tD\t\tE\t\tF\t\tG\t\tH\t\t");
}
fn main() {
    println!("hello world ! I'm a Rustacean ! ");
    welcome();
    //welcome fn (explains the basic inputs to user)
    //board class/implemenation fn to trigger initial board
    board();
    

}

