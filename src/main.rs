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

    let rook = pieces::rook::Rook::new(util::color::Color::black(), util::position::Position::new(String::from("A1")));
    let pawn = pieces::pawn::Pawn::new(util::color::Color::white(), util::position::Position::new(String::from("A2")));
    let knight = pieces::knight::Knight::new(util::color::Color::black(), util::position::Position::new(String::from("A3")));
    let bishop = pieces::bishop::Bishop::new(util::color::Color::white(), util::position::Position::new(String::from("A4")));
    let king = pieces::king::King::new(util::color::Color::black(), util::position::Position::new(String::from("A5")));
    let queen = pieces::queen::Queen::new(util::color::Color::white(), util::position::Position::new(String::from("A6")));

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
