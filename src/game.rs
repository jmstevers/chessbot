use crate::board::Board;
use crate::piece::PieceType;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game { board }
    }

    pub fn parse_move(&self, msg: &str) -> Result<((usize, usize), (usize, usize)), String> {
        let chars = msg.chars().collect::<Vec<_>>();
        let mut from = (99 as usize, 99 as usize);

        let piece_type = match chars[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => PieceType::Pawn,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => return Err("First character is invalid.".into()),
        };

        // if self.board.num_of_type_in_row(piece_type, row) > 1 {
        //     todo!()
        // } else if self.board.num_of_type_in_col(piece_type, col)

        let to = (
            0 as usize,
            match chars.last().unwrap().to_digit(10) {
                Some(x) => x as usize,
                None => return Err("Last character is not a number.".into()),
            },
        );

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn white_pawn_move_one() {
        let board = Board::new();
        let game = Game::new(board);
        let msg = "e4";

        match game.parse_move(msg) {
            Ok(pos) => assert_eq!(((6, 4), (4, 4)), pos),
            Err(err) => {
                println!("{err}");
                assert!(false)
            }
        };
    }
}
