struct Board{
    size: u32,
}

enum Piece{
    K,
    Q,
    R,
    B,
    N,
}

impl Board{
    fn new() -> Board{
        Board{
            size: 8,
        }
    }

    fn transform_algebraic_notation(&self, algebraic_notation: &str) -> (char, char){
        if algebraic_notation.len() != 2{
            panic!("Invalid algebraic notation");
        }

        let mut list: [char] = algebraic_notation.chars().collect();
        (list[0], list[1])
    }
}

//  a b c d e f g h
//8 w b w b w b w b 8
//7 b w b w b w b w 7
//6 w b w b w b w b 6
//5 b w b w b w b w 5
//4 w b w b w b w b 4
//3 b w b w b w b w 3
//2 w b w b w b w b 2
//1 b w b w b w b w 1
//  a b c d e f g h