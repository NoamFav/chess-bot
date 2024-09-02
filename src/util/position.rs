#[derive(Debug)]
pub(crate) struct Position {
    x: char,
    y: char
}

impl Position {
    pub(crate) fn new(y: char, x:char) -> Self {
        Self {
            x,
            y
        }
    }
}