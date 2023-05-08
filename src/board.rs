use crate::piece::*;
use std::fmt::Display;
use unroll::unroll_for_loops;

pub struct Board<'a> {
    tiles: [&'a Piece; 64],
    turn: i32,
}

impl Board<'_> {
    pub const fn new() -> Board<'static> {
        let tiles = [
            &Piece::Rook(Color::Black),
            &Piece::Knight(Color::Black),
            &Piece::Bishop(Color::Black),
            &Piece::Queen(Color::Black),
            &Piece::King(Color::Black),
            &Piece::Bishop(Color::Black),
            &Piece::Knight(Color::Black),
            &Piece::Rook(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Pawn(Color::Black),
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Empty,
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Pawn(Color::White),
            &Piece::Rook(Color::White),
            &Piece::Knight(Color::White),
            &Piece::Bishop(Color::White),
            &Piece::Queen(Color::White),
            &Piece::King(Color::White),
            &Piece::Bishop(Color::White),
            &Piece::Knight(Color::White),
            &Piece::Rook(Color::White),
        ];
        Board { tiles, turn: 1 }
    }

    pub fn get_from_col(&self, col: i32, piece: Piece) -> Vec<Position> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(index, x)| (index + col as usize) % 8 == 0 && ***x == piece)
            .map(|(index, _)| Position((index - (index % 8) / 8) as i32, (index % 8) as i32))
            .collect()
    }

    pub fn get_from_row(&self, row: i32, piece: Piece) -> Vec<Position> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(index, x)| index % 8 == row as usize && ***x == piece)
            .map(|(index, _)| Position((index - (index % 8) / 8) as i32, (index % 8) as i32))
            .collect()
    }

    pub fn turn_color(&self) -> Color {
        // wildcard literally can not happen
        match self.turn % 2 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!(),
        }
    }

    pub fn count_same_type_from_possible_moves(
        &self,
        piece: &Piece,
        from: Position,
        take: bool,
    ) -> Option<usize> {
        let possible_positions = piece.possible_moves(from, take)?;
        let result = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(x, y)| {
                println!("{:?} {:?}", ***y, *piece);
                possible_positions
                    .iter()
                    .for_each(|possible| println!("{} {}", *x as i32, 8 * possible.0 + possible.1));
                ***y == *piece
                    && possible_positions
                        .iter()
                        .any(|possible| (*x as i32) == (8 * possible.0 + possible.1))
            })
            .count();
        Some(result)
    }

    pub fn num_of_type_in_row(&self, piece: Piece, row: usize) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(index, x)| index % 8 == row && ***x == piece)
            .count()
    }

    pub fn num_of_type_in_col(&self, piece: Piece, col: usize) -> usize {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(index, x)| (index + col) % 8 == 0 && ***x == piece)
            .count()
    }

    pub fn move_piece(&mut self, from: Position, to: Position, take: bool) -> Result<(), String> {
        let piece = self.tiles[(8 * from.0 + from.1) as usize];

        if let Some(moves) = piece.possible_moves(from, take) && moves.contains(&to) {
            self.tiles[(8 * from.0 + from.1) as usize] = &Piece::Empty;
            self.tiles[(8 * from.0 + from.1) as usize] = piece;
            Ok(())
        } else {
            return Err("Piece has no possible moves.".into());
        }
    }
}

#[unroll_for_loops]
impl Display for Board<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.tiles[(8 * i + j) as usize];
                board_string.push_str(&(String::from(piece.to_char()) + " "));
            }
            board_string.push_str("\n")
        }

        write!(f, "{board_string}")
    }
}
