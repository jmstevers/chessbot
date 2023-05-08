use crate::{board::Board, piece::*};
use regex::Regex;

pub struct Game<'a> {
    board: Board<'a>,
}

impl Game<'_> {
    pub fn new(board: Board) -> Game {
        Game { board }
    }

    // returns from position, to position, if it takes, it it promotes and if it checks/checkmates
    pub fn parse_move(&self, msg: &str) -> Result<(Position, Position, bool, bool, bool), String> {
        let re = match Regex::new(
            r"(?:([PNBRQK])?([a-h]?[1-8]?)?(x)?([a-h][1-8])(?:(=)?([NBRQ])?)?|O(-?O){1,2})([\+#])?",
        ) {
            Ok(x) => x,
            Err(x) => return Err(x.to_string()),
        };

        let binding = re.captures(msg).unwrap();
        let caps = binding
            .iter()
            .map(|x| match x {
                Some(x) => Some(x.as_str()),
                None => None,
            })
            .collect::<Vec<_>>();

        let color = self.board.turn_color();

        let take = caps[3].is_some_and(|x| x == "x");
        let checks = caps[8].is_some_and(|x| x == "+" || x == "#");
        let promote = caps[5].is_some_and(|x| x == "=");

        let piece = match caps[1] {
            Some(x) => Piece::from_str(x, color).unwrap(),
            None => Piece::Pawn(color),
        };

        let to = Position::from_str(caps[4].unwrap()).unwrap();

        let from = if let Some(num) = self
            .board
            .count_same_type_from_possible_moves(&piece, to, take)
        {
            if num < 1 {
                return Err(format!("No {:?} can move to {}", &piece, caps[4].unwrap()));
            } else {
                let from_str = caps[2].unwrap();
                match from_str.len() {
                    // wildcard can't happen
                    1 => match from_str {
                        "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" => self
                            .board
                            .get_from_col(str_to_num(from_str).unwrap(), piece)[0],
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" => {
                            self.board.get_from_row(parse_num(from_str).unwrap(), piece)[0]
                        }
                        _ => panic!(),
                    },
                    _ => Position::from_str(from_str).unwrap(),
                }
            }
        } else {
            return Err("Attempted to move an empty square.".into());
        };

        Ok((from, to, take, promote, checks))
    }
}

fn str_to_num(msg: &str) -> Result<i32, String> {
    match msg {
        "a" => Ok(0),
        "b" => Ok(1),
        "c" => Ok(2),
        "d" => Ok(3),
        "e" => Ok(4),
        "f" => Ok(5),
        "g" => Ok(6),
        "h" => Ok(7),
        _ => return Err("Col is invalid.".into()),
    }
}

fn parse_num(msg: &str) -> Result<i32, String> {
    match msg.chars().collect::<Vec<_>>()[0].to_digit(10) {
        Some(x) => Ok(x as i32 - 1),
        None => return Err("Row is invalid.".into()),
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
            Ok(x) => {
                println!("{x:?}");
                assert_eq!(x, (Position(6, 4), Position(4, 4), false, false, false))
            }
            Err(x) => {
                println!("{x}");
                panic!()
            }
        }
    }
}
