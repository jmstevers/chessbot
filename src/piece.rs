use crate::board::Board;

enum PieceType {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    type_of: PieceType,
    position: (usize, usize),
    color: bool,
}

impl Piece {
    pub fn move_to(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        if to.0 > 8 || to.1 > 8 {
            return Err("Invalid board square (out of bounds)".into());
        }

        match self.type_of {
            PieceType::Empty => Err("Can not move empty square".into()),
            PieceType::Pawn => self.move_pawn(board, to, take),
            PieceType::Knight => self.move_knight(board, to, take),
            PieceType::Bishop => self.move_bishop(board, to, take),
            PieceType::Rook => self.move_rook(board, to, take),
            PieceType::Queen => self.move_queen(board, to, take),
            PieceType::King => self.move_king(board, to, take),
        }
    }

    fn move_pawn(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_knight(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_bishop(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_rook(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_queen(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_king(&mut self, board: Board, to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }
}
