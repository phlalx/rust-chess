
enum PieceKind { Pawn, Knight, Bishop, Rook, Queen, King }

fn cost(piece: PieceKind) -> u32 {
    match piece {
        PieceKind::Pawn => 1,
        PieceKind::Knight => 3,
        PieceKind::Bishop => 3,
        PieceKind::Rook => 5,
        PieceKind::Queen => 9,
        PieceKind::King => 1000, // TODO max_int?
    }
}

fn to_char(piece: PieceKind) -> char {
    // TODO use color
    match piece {
        PieceKind::Pawn => 'P',
        PieceKind::Knight => 'K',
        PieceKind::Bishop => 'B',
        PieceKind::Rook => 'R',
        PieceKind::Queen => 'Q',
        PieceKind::King => 'K',
    }
}

struct Piece {
    pos: crate::position::Position,
    color: crate::color::Color,
    kind: PieceKind,
    is_captured: bool,
}

