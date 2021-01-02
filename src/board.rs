use crate::color;
use crate::piece;
use crate::position;

struct Board {
    board: Vec<piece::Piece>,
}

fn add_piece(
    vec: &mut Vec<piece::Piece>,
    i: u8,
    j: u8,
    kind: piece::PieceKind,
    color: color::Color,
) {
    let piece: piece::Piece = piece::Piece::new(position::Position(i, j), color, kind);
    vec.push(piece);
}

impl Board {
    fn new() -> Board {
        let mut v: Vec<piece::Piece> = Vec::new();
        add_piece(&mut v, 0, 0, piece::PieceKind::Rook, color::Color::White);
        add_piece(&mut v, 0, 1, piece::PieceKind::Knight, color::Color::White);
        add_piece(&mut v, 0, 2, piece::PieceKind::Bishop, color::Color::White);
        add_piece(&mut v, 0, 3, piece::PieceKind::Queen, color::Color::White);
        add_piece(&mut v, 0, 4, piece::PieceKind::King, color::Color::White);
        add_piece(&mut v, 0, 5, piece::PieceKind::Bishop, color::Color::White);
        add_piece(&mut v, 0, 6, piece::PieceKind::Knight, color::Color::White);
        add_piece(&mut v, 0, 7, piece::PieceKind::Rook, color::Color::White);
        add_piece(&mut v, 7, 0, piece::PieceKind::Rook, color::Color::Black);
        add_piece(&mut v, 7, 1, piece::PieceKind::Knight, color::Color::Black);
        add_piece(&mut v, 7, 2, piece::PieceKind::Bishop, color::Color::Black);
        add_piece(&mut v, 7, 3, piece::PieceKind::Queen, color::Color::Black);
        add_piece(&mut v, 7, 4, piece::PieceKind::King, color::Color::Black);
        add_piece(&mut v, 7, 5, piece::PieceKind::Bishop, color::Color::Black);
        add_piece(&mut v, 7, 6, piece::PieceKind::Knight, color::Color::Black);
        add_piece(&mut v, 7, 7, piece::PieceKind::Rook, color::Color::Black);
        for i in 0..7 {
            add_piece(&mut v, i, 1, piece::PieceKind::Pawn, color::Color::White);
            add_piece(&mut v, i, 6, piece::PieceKind::Pawn, color::Color::Black);
        }
        Board { board: v }
    }
}
