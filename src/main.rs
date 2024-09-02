use util::color::Color;
use util::position::Position;

mod pieces {
    pub mod rook;
    pub mod pawn;
    pub mod knight;
    pub mod bishop;
    pub mod king;
    pub mod queen;
}

mod util {
    pub mod color;
    pub mod position;
    pub mod board;
}

fn main() {

    let rook = pieces::rook::Rook::new(Color::black(), Position::new('a', '1'));
    let pawn = pieces::pawn::Pawn::new(Color::white(), Position::new('a', '2'));
    let knight = pieces::knight::Knight::new(Color::black(), Position::new('a', '3'));
    let bishop = pieces::bishop::Bishop::new(Color::white(), Position::new('a', '4'));
    let king = pieces::king::King::new(Color::black(), Position::new('a', '5'));
    let queen = pieces::queen::Queen::new(Color::white(), Position::new('a', '6'));

    println!("{:?}", rook.color);
    println!("{:?}", rook.position);
    println!("{:?}", rook.has_moved);

    println!();

    println!("{:?}", pawn.color);
    print!("{:?}", pawn.position);
    println!("{:?}", pawn.has_moved);

    println!();

    println!("{:?}", knight.color);
    println!("{:?}", knight.position);
    println!("{:?}", knight.has_moved);

    println!();

    println!("{:?}", bishop.color);
    println!("{:?}", bishop.position);
    println!("{:?}", bishop.has_moved);

    println!();

    println!("{:?}", king.color);
    print!("{:?}", king.position);
    println!("{:?}", king.has_moved);

    println!();

    println!("{:?}", queen.color);
    print!("{:?}", queen.position);
    println!("{:?}", queen.has_moved);

    println!("Hello, world!");
}
