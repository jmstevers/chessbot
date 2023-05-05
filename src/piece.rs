use crate::board::Board;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Self::Pawn => self.pawn_possible_moves(board, position),
            Self::Knight => self.knight_possible_moves(board, position),
            Self::Bishop => self.bishop_possible_moves(board, position),
            Self::Rook => self.rook_possible_moves(board, position),
            Self::Queen => self.queen_possible_moves(board, position),
            Self::King => self.king_possible_moves(board, position),
        }
    }

    fn pawn_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn knight_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn bishop_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn rook_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn queen_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }

    fn king_possible_moves(&self, board: Board, position: (usize, usize)) -> Vec<(usize, usize)> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub const fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }

    pub fn to_str(&self) -> &str {
        match self.piece_type {
            PieceType::Pawn => match self.color {
                Color::White => "♙",
                Color::Black => "♟︎",
            },
            PieceType::Knight => match self.color {
                Color::White => "♘",
                Color::Black => "♞",
            },
            PieceType::Bishop => match self.color {
                Color::White => "♗",
                Color::Black => "♝",
            },
            PieceType::Rook => match self.color {
                Color::White => "♖",
                Color::Black => "♜",
            },
            PieceType::Queen => match self.color {
                Color::White => "♕",
                Color::Black => "♛",
            },
            PieceType::King => match self.color {
                Color::White => "♔",
                Color::Black => "♚",
            },
        }
    }
}
