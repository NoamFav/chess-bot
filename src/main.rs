use crate::util::board::Board;
use crate::util::pieces;
use util::color::Color;
use util::position::Position;

mod util {
    pub mod board;
    pub mod color;
    pub mod pieces;
    pub mod position;
}

pub mod thor;

fn main() {
    let _rook = pieces::Rook::new(Color::black(), Position::new('a', '1'));
    let _pawn = pieces::Pawn::new(Color::white(), Position::new('a', '2'));
    let _knight = pieces::Knight::new(Color::black(), Position::new('a', '3'));
    let _bishop = pieces::Bishop::new(Color::white(), Position::new('a', '4'));
    let _king = pieces::King::new(Color::black(), Position::new('a', '5'));
    let _queen = pieces::Queen::new(Color::white(), Position::new('a', '6'));

    let mut board = Board::new();
    println!("{}", board.render_ascii());

    println!("{:?}", board.transform_algebraic_notation("e4"));
    println!("{:?}", board.transform_algebraic_notation("e5"));
    println!("{:?}", board.transform_algebraic_notation("Nf3"));
    println!("{:?}", board.transform_algebraic_notation("Nc6"));
    println!("{:?}", board.transform_algebraic_notation("Bb5"));
    println!("{:?}", board.transform_algebraic_notation("a6"));
    println!("{:?}", board.transform_algebraic_notation("Ba4+"));
    println!("{:?}", board.transform_algebraic_notation("Nf6"));
    println!("Hello, world!");
}
