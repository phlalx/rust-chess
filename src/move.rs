use crate::piece;
use crate::position;

enum Move {
    Basic {
        piece: piece::Piece,
        to: position::Position,
    },
    Capture {
        piece: piece::Piece,
        captured: piece::Piece,
    },
}
