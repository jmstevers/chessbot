use crate::piece::{Piece, PieceType, Color};

pub struct Board {
    tiles: [[Piece; 8]; 8],
}

impl Board {
    // pub fn new() -> Board {
    //     let tiles = [[Piece::new(PieceType::Rook, Color::Black)],[],[],[],[],[],[],[]]
    // }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        if to.0 > 8 || to.1 > 8  {
            return Err("Invalid board square (out of bounds)".into());
        }

        match self.tiles[from.0][from.1].piece_type {
            PieceType::Empty => Err("Can not move empty square".into()),
            PieceType::Pawn => self.move_pawn(from, to, take),
            PieceType::Knight => self.move_knight(from, to, take),
            PieceType::Bishop => self.move_bishop(from, to, take),
            PieceType::Rook => self.move_rook(from, to, take),
            PieceType::Queen => self.move_queen(from, to, take),
            PieceType::King => self.move_king(from, to, take),
        }
    }

    fn move_pawn(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        let delta = (to.0 - from.0, to.1 - from.1);

        println!("{delta:?}");

        match (self.tiles[from.0][from.1].color, take) {
            (Color::White, false) => {
                if to.1 < from.1 {
                    return Err("Can't move pawn backwards.".into())
                }
                todo!()
            },
            (Color::Black, false) => {
                if to.1 > from.1 {
                    return Err("Can't move pawn backwards.".into())
                }
                todo!()
            }
            (Color::White, true) => {
                if to.1 < from.1 {
                    return Err("Pawn can't take backwards.".into())
                }
                todo!()
            },
            (Color::Black, true) => {
                if to.1 > from.1 {
                    return Err("Pawn can't take backwards.".into())
                }
                todo!()
            }
        }
    }

    fn move_knight(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_bishop(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_rook(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_queen(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }

    fn move_king(&mut self, from: (usize, usize), to: (usize, usize), take: bool) -> Result<(), String> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pawn() {
        move_pawn()
    }
}