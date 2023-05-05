use crate::piece::{Color, Piece, PieceType};
use std::fmt::Display;
use unroll::unroll_for_loops;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Error {
    InvalidMove,
}

pub struct Board {
    tiles: [[Option<Piece>; 8]; 8],
    turn: u32,
}

impl Board {
    pub fn new() -> Board {
        let tiles = [
            [
                Some(Piece::new(PieceType::Rook, Color::Black)),
                Some(Piece::new(PieceType::Knight, Color::Black)),
                Some(Piece::new(PieceType::Bishop, Color::Black)),
                Some(Piece::new(PieceType::Queen, Color::Black)),
                Some(Piece::new(PieceType::King, Color::Black)),
                Some(Piece::new(PieceType::Bishop, Color::Black)),
                Some(Piece::new(PieceType::Knight, Color::Black)),
                Some(Piece::new(PieceType::Rook, Color::Black)),
            ],
            [const { Some(Piece::new(PieceType::Pawn, Color::Black)) }; 8],
            [const { None }; 8],
            [const { None }; 8],
            [const { None }; 8],
            [const { None }; 8],
            [const { Some(Piece::new(PieceType::Pawn, Color::White)) }; 8],
            [
                Some(Piece::new(PieceType::Rook, Color::White)),
                Some(Piece::new(PieceType::Knight, Color::White)),
                Some(Piece::new(PieceType::Bishop, Color::White)),
                Some(Piece::new(PieceType::Queen, Color::White)),
                Some(Piece::new(PieceType::King, Color::White)),
                Some(Piece::new(PieceType::Bishop, Color::White)),
                Some(Piece::new(PieceType::Knight, Color::White)),
                Some(Piece::new(PieceType::Rook, Color::White)),
            ],
        ];
        Board { tiles, turn: 0 }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Piece> {
        self.tiles[row - 1][col - 1].as_ref()
    }

    pub fn num_of_type_in_row(&self, piece_type: PieceType, row: usize) -> usize {
        self.tiles[row]
            .iter()
            .filter_map(|x| x.as_ref())
            .filter(|x| x.piece_type == piece_type)
            .count()
    }

    pub fn num_of_type_in_col(&self, piece_type: PieceType, col: usize) -> u8 {
        let flat = self.tiles.iter().flatten().collect::<Vec<_>>();

        let mut total = 0;

        let mut count = col;
        while count < 64 {
            if let Some(piece) = flat[count] && piece.piece_type == piece_type{
                total += 1;
            }

            count += 8;
        }

        total
    }

    pub fn move_piece(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
    ) -> Result<(), (Error, String)> {
        if to.0 > 8 || to.1 > 8 {
            return Err((
                Error::InvalidMove,
                "Invalid board square (out of bounds)".into(),
            ));
        }

        if let Some(piece) = self.get(from.0, from.1) {
            let color = piece.color;
            let result = match piece.piece_type {
                PieceType::Pawn => self.move_pawn(from, to, take, color),
                PieceType::Knight => self.move_knight(from, to, take, color),
                PieceType::Bishop => self.move_bishop(from, to, take, color),
                PieceType::Rook => self.move_rook(from, to, take, color),
                PieceType::Queen => self.move_queen(from, to, take, color),
                PieceType::King => self.move_king(from, to, take, color),
            };
            self.tiles[from.0][from.1] = None;
            result
        } else {
            return Err((
                Error::InvalidMove,
                "Attempted to move an empty square".into(),
            ));
        }
    }

    fn move_pawn(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        self.tiles[to.0][to.1] = Some(Piece::new(PieceType::Pawn, color));
        Ok(())
    }

    fn move_knight(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        todo!()
    }

    fn move_bishop(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        todo!()
    }

    fn move_rook(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        todo!()
    }

    fn move_queen(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        todo!()
    }

    fn move_king(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        take: bool,
        color: Color,
    ) -> Result<(), (Error, String)> {
        todo!()
    }
}

#[unroll_for_loops]
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &self.tiles[i][j] {
                    board_string.push_str(&(String::from(piece.to_str()) + " "));
                } else {
                    board_string.push_str("• ");
                }
            }
            board_string.push_str("\n")
        }

        write!(f, "{board_string}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn legal_pawn_move() {
        let mut board = Board::new();
        let m = board.move_piece((6, 4), (4, 4), false);
        println!("{board}");

        match m {
            Ok(_) => (),
            Err((err, ref msg)) => println!("Err: {err:?}\nMessage: {msg}"),
        }

        assert!(m.is_ok())
    }

    #[test]
    fn legal_pawn_take() {
        let mut board = Board::new();
        let m = board.move_piece((6, 4), (4, 4), true);
        println!("{board}");

        match m {
            Ok(_) => (),
            Err((err, ref msg)) => println!("Err: {err:?}\nMessage: {msg}"),
        }

        assert!(m.is_ok())
    }

    #[test]
    fn illegal_pawn_move() {
        let mut board = Board::new();
        let m = board.move_piece((6, 4), (4, 4), false);
        println!("{board}");

        match m {
            Ok(_) => (),
            Err((err, ref msg)) => println!("Err: {err:?}\nMessage: {msg}"),
        }

        assert!(m.is_err());
        assert!(m.is_err_and(|x| x.0 == Error::InvalidMove));
    }

    #[test]
    fn illegal_pawn_take() {
        let mut board = Board::new();
        let m = board.move_piece((6, 4), (7, 4), true);
        println!("{board}");

        match m {
            Ok(_) => (),
            Err((err, ref msg)) => println!("Err: {err:?}\nMessage: {msg}"),
        }

        assert!(m.is_err());
        assert!(m.is_err_and(|x| x.0 == Error::InvalidMove));
    }

    #[test]
    fn num_in_row() {
        let board = Board::new();
        let count = board.num_of_type_in_row(PieceType::Pawn, 1);

        assert_eq!(8, count)
    }

    #[test]
    fn num_in_col() {
        let mut board = Board::new();
        board.move_piece((1, 1), (2, 0), false).unwrap();
        let count = board.num_of_type_in_col(PieceType::Rook, 0);

        assert_eq!(2, count)
    }
}
