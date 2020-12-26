use crate::position;
use crate::piece;

enum Move {
    Basic {piece: piece::Piece,
           to:position::Position},
    Capture {piece: piece::Piece,
             captured: piece::Piece}
}
