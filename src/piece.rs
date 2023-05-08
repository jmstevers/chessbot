use std::ops::{Add, Neg};

#[derive(PartialEq, Debug)]
pub enum Piece {
    Empty,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self {
            Piece::Pawn(color) => match color {
                Color::White => 'P',
                Color::Black => 'P',
            },
            Piece::Knight(color) => match color {
                Color::White => 'K',
                Color::Black => 'K',
            },
            Piece::Bishop(color) => match color {
                Color::White => 'B',
                Color::Black => 'B',
            },
            Piece::Rook(color) => match color {
                Color::White => 'R',
                Color::Black => 'R',
            },
            Piece::Queen(color) => match color {
                Color::White => 'Q',
                Color::Black => 'Q',
            },
            Piece::King(color) => match color {
                Color::White => 'K',
                Color::Black => 'K',
            },
            Piece::Empty => '.',
        }
    }

    pub fn from_str(msg: &str, color: Color) -> Result<Piece, String> {
        match msg {
            "P" => Ok(Piece::Pawn(color)),
            "N" => Ok(Piece::Knight(color)),
            "B" => Ok(Piece::Bishop(color)),
            "R" => Ok(Piece::Rook(color)),
            "Q" => Ok(Piece::Queen(color)),
            "K" => Ok(Piece::King(color)),
            _ => return Err("Failed to parse string to piece.".into()),
        }
    }
    pub fn possible_moves(&self, from: Position, take: bool) -> Option<Vec<Position>> {
        match self {
            Piece::Pawn(color) => Some(self.pawn_possible_moves(*color, from, take)),
            Piece::Knight(_) => Some(self.knight_possible_moves(from)),
            Piece::Bishop(_) => Some(self.bishop_possible_moves(from)),
            Piece::Rook(_) => Some(self.rook_possible_moves(from)),
            Piece::Queen(_) => Some(self.queen_possible_moves(from)),
            Piece::King(_) => Some(self.king_possible_moves(from)),
            Piece::Empty => None,
        }
    }

    fn pawn_possible_moves(&self, color: Color, from: Position, take: bool) -> Vec<Position> {
        let mut rules = vec![Position(1, 0), Position(2, 0)];
        let take_rules = vec![Position(1, -1), Position(1, 1)];
        rules.extend(take_rules.iter());

        let tos = rules.iter().map(|x| x + &from).collect();

        match color {
            Color::White => tos,
            Color::Black => tos.iter().map(|x| -x).collect(),
        }
    }

    fn knight_possible_moves(&self, from: Position) -> Vec<Position> {
        let rules = vec![
            Position(2, 1),
            Position(1, 2),
            Position(-2, 1),
            Position(-1, 2),
            Position(-2, -1),
            Position(-1, -2),
            Position(2, -1),
            Position(1, -2),
        ];

        rules.iter().map(|x| x + &from).collect()
    }

    fn bishop_possible_moves(&self, from: Position) -> Vec<Position> {
        let rules = vec![
            Position(1, 1),
            Position(2, 2),
            Position(3, 3),
            Position(4, 4),
            Position(5, 5),
            Position(6, 6),
            Position(7, 7),
            Position(-1, 1),
            Position(-2, 2),
            Position(-3, 3),
            Position(-4, 4),
            Position(-5, 5),
            Position(-6, 6),
            Position(-7, 7),
            Position(1, -1),
            Position(2, -2),
            Position(3, -3),
            Position(4, -4),
            Position(5, -5),
            Position(6, -6),
            Position(7, -7),
            Position(-1, -1),
            Position(-2, -2),
            Position(-3, -3),
            Position(-4, -4),
            Position(-5, -5),
            Position(-6, -6),
            Position(-7, -7),
        ];

        rules.iter().map(|x| x + &from).collect()
    }

    fn rook_possible_moves(&self, from: Position) -> Vec<Position> {
        let rules = vec![
            Position(0, 1),
            Position(0, 2),
            Position(0, 3),
            Position(0, 4),
            Position(0, 5),
            Position(0, 6),
            Position(0, 7),
            Position(0, -1),
            Position(0, -2),
            Position(0, -3),
            Position(0, -4),
            Position(0, -5),
            Position(0, -6),
            Position(0, -7),
            Position(1, 0),
            Position(2, 0),
            Position(3, 0),
            Position(4, 0),
            Position(5, 0),
            Position(6, 0),
            Position(7, 0),
            Position(-1, 0),
            Position(-2, 0),
            Position(-3, 0),
            Position(-4, 0),
            Position(-5, 0),
            Position(-6, 0),
            Position(-7, 0),
        ];

        rules.iter().map(|x| x + &from).collect()
    }

    fn queen_possible_moves(&self, from: Position) -> Vec<Position> {
        let rook_moves = self.rook_possible_moves(from);
        let bishop_moves = self.bishop_possible_moves(from);
        let rules = rook_moves
            .iter()
            .chain(bishop_moves.iter())
            .collect::<Vec<_>>();

        rules.iter().map(|x| *x + &from).collect()
    }

    fn king_possible_moves(&self, from: Position) -> Vec<Position> {
        let rules = vec![
            Position(1, 0),
            Position(1, 1),
            Position(0, 1),
            Position(-1, 1),
            Position(-1, 0),
            Position(-1, -1),
            Position(0, -1),
            Position(1, -1),
        ];

        rules.iter().map(|x| x + &from).collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn from_str(msg: &str) -> Result<Position, String> {
        let chars = msg.chars().collect::<Vec<_>>();
        let y = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err("Col is invalid.".into()),
        };

        let x = match chars[1].to_digit(10) {
            Some(x) => x as i32 - 1,
            None => return Err("Row is invalid character.".into()),
        };

        Ok(Position(x, y))
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Position(-self.0, -self.1)
    }
}

impl Neg for &Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position(-self.0, -self.1)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
