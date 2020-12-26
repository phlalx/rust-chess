use crate::piece;

struct Board {
    board: Vec<piece::Piece>,
}

impl Board {
    fn new() -> Board {
        let v : Vec<piece::Piece> = Vec::new();
        Board{board: v}
    }

}