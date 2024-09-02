use crate::util::color::Color;
use crate::util::position::Position;
use crate::util::pieces::{Rook, Knight, Bishop, Queen, King, Pawn};
use crate::util::pieces::Piece;

pub(crate) struct Board{
    pub(crate) size: u32,
    pub(crate) is_white_turn: bool,
    pub(crate) pieces: Vec<Box<dyn Piece>>,
}

#[derive(Debug)]
pub(crate) struct Move{
    pub(crate) piece: char, // K, Q, R, B, N, P
    pub(crate) x: char, // a-h
    pub(crate) y: char, // 1-8
    pub(crate) is_capture: bool,
    pub(crate) is_check: bool,
    pub(crate) is_checkmate: bool,
    pub(crate) is_white: bool,
}

impl Board{
    pub(crate) fn new() -> Board {
        let mut pieces: Vec<Box<dyn Piece>> = Vec::new();

        // Add white pieces
        pieces.push(Box::new(Rook::new(Color::white(), Position::new('a', '1'))));
        pieces.push(Box::new(Knight::new(Color::white(), Position::new('b', '1'))));
        pieces.push(Box::new(Bishop::new(Color::white(), Position::new('c', '1'))));
        pieces.push(Box::new(Queen::new(Color::white(), Position::new('d', '1'))));
        pieces.push(Box::new(King::new(Color::white(), Position::new('e', '1'))));
        pieces.push(Box::new(Bishop::new(Color::white(), Position::new('f', '1'))));
        pieces.push(Box::new(Knight::new(Color::white(), Position::new('g', '1'))));
        pieces.push(Box::new(Rook::new(Color::white(), Position::new('h', '1'))));
        for i in 0..8 {
            pieces.push(Box::new(Pawn::new(Color::white(), Position::new(('a' as u8 + i) as char, '2'))));
        }

        // Add black pieces
        pieces.push(Box::new(Rook::new(Color::black(), Position::new('a', '8'))));
        pieces.push(Box::new(Knight::new(Color::black(), Position::new('b', '8'))));
        pieces.push(Box::new(Bishop::new(Color::black(), Position::new('c', '8'))));
        pieces.push(Box::new(Queen::new(Color::black(), Position::new('d', '8'))));
        pieces.push(Box::new(King::new(Color::black(), Position::new('e', '8'))));
        pieces.push(Box::new(Bishop::new(Color::black(), Position::new('f', '8'))));
        pieces.push(Box::new(Knight::new(Color::black(), Position::new('g', '8'))));
        pieces.push(Box::new(Rook::new(Color::black(), Position::new('h', '8'))));
        for i in 0..8 {
            pieces.push(Box::new(Pawn::new(Color::black(), Position::new(('a' as u8 + i) as char, '7'))));
        }

        Board {
            size: 8,
            is_white_turn: true,
            pieces,
        }
    }

    pub(crate) fn transform_algebraic_notation(&self, algebraic_notation: &str) -> Move {
        if algebraic_notation.len() < 2 {
            panic!("Invalid algebraic notation");
        }

        let mut list: Vec<char> = algebraic_notation.chars().collect();

        // x, y, piece, is_capture, is_check, is_checkmate
        let mut return_value: Move = Move {
            piece: 'P',
            x: ' ',
            y: ' ',
            is_capture: false,
            is_check: false,
            is_checkmate: false,
            is_white: self.is_white_turn,
        };

        // Handle castling notation
        if algebraic_notation == "O-O" && self.is_white_turn {
            // TODO: implement castling check (may be invalid)
            panic!("Castling not implemented");
        } else if algebraic_notation == "O-O-O" && self.is_white_turn {
            // TODO: implement castling check (may be invalid)
            panic!("Long castling not implemented");
        }

        // Handle capture notation and check/checkmate
        if let Some(pos) = list.iter().position(|&x| x == '+') {
            return_value.is_capture = true;
            list.remove(pos);
        } else if let Some(pos) = list.iter().position(|&x| x == '#') {
            return_value.is_checkmate = true;
            list.remove(pos);
        } else if let Some(pos) = list.iter().position(|&x| x == 'x') {
            return_value.is_check = true;
            list.remove(pos);
        }

        // Handle piece notation
        if list[0].is_uppercase() {
            let is_valid = match list[0] {
                'K' | 'Q' | 'R' | 'B' | 'N' => list[0],
                _ => 'P',
            };

            return_value.piece = is_valid;
            list.remove(0);
        }

        // Handle pawn promotion
        if let Some(pos) = list.iter().position(|&x| x == '=') {
            if pos + 1 < list.len() {
                return_value.piece = list[pos + 1];
                list.drain(pos..=pos + 1);
            } else {
                panic!("Invalid promotion notation");
            }
        }

        // Handle ambiguous moves (e.g., Nbd2 or R1e2)
        if list.len() == 3 {
            if list[1].is_alphabetic() && list[2].is_numeric() {
                return_value.x = list[1];
                return_value.y = list[2];
                list.remove(1);
                list.remove(1);
            } else if list[1].is_numeric() && list[2].is_alphabetic() {
                return_value.x = list[2];
                return_value.y = list[1];
                list.remove(1);
                list.remove(1);
            }
        }

        for &ch in &list {
            if ch.is_alphabetic() {
                return_value.x = ch;
            } else if ch.is_numeric() {
                return_value.y = ch;
            }
        }

        return_value
    }

    // Method to render the board in ASCII
    // Not mine, taken from gpt as a temporary solution to see if the board is being saved correctly
    pub(crate) fn render_ascii(&self) -> String {
        let mut board_representation = [['.'; 8]; 8]; // 8x8 board initialized with dots

        // Iterate over pieces and place their symbols on the board
        for piece in &self.pieces {
            let pos = piece.position();
            let x = pos.x as usize - 'a' as usize; // Convert column character to 0-indexed (a-h to 0-7)
            let y_index = pos.y as usize - '1' as usize; // Convert row character to 0-indexed (1-8 to 0-7)

            // Ensure valid position calculations
            if y_index >= 8 {
                panic!("Invalid row position for chess piece: {}", pos.y);
            }

            let y = 7 - y_index; // Invert row to align with visual representation of the board

            board_representation[y][x] = match piece.as_any().downcast_ref::<Rook>() {
                Some(_) if piece.color().is_white() => '♜',
                Some(_) => '♖',
                None => match piece.as_any().downcast_ref::<Knight>() {
                    Some(_) if piece.color().is_white() => '♞',
                    Some(_) => '♘',
                    None => match piece.as_any().downcast_ref::<Bishop>() {
                        Some(_) if piece.color().is_white() => '♝',
                        Some(_) => '♗',
                        None => match piece.as_any().downcast_ref::<Queen>() {
                            Some(_) if piece.color().is_white() => '♛',
                            Some(_) => '♕',
                            None => match piece.as_any().downcast_ref::<King>() {
                                Some(_) if piece.color().is_white() => '♚',
                                Some(_) => '♔',
                                None => match piece.as_any().downcast_ref::<Pawn>() {
                                    Some(_) if piece.color().is_white() => '♟',
                                    Some(_) => '♙',
                                    None => '.',
                                },
                            },
                        },
                    },
                },
            };
        }

        // Convert board array to a single string for display
        let mut board_string = String::new();
        board_string.push_str("  a b c d e f g h\n"); // Header with column labels
        for (i, row) in board_representation.iter().enumerate() {
            board_string.push_str(&format!("{} ", 8 - i)); // row numbers starting from 8 at the top
            for &cell in row {
                board_string.push(cell);
                board_string.push(' ');
            }
            board_string.push_str(&format!(" {}\n", 8 - i)); // row numbers repeated at the end of the line
        }
        board_string.push_str("  a b c d e f g h\n"); // Footer with column labels

        board_string
    }
}