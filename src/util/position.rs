#[derive(Debug)]
pub(crate) struct Position {
    pub(crate) x: char,
    pub(crate) y: char
}

impl Position {
    pub(crate) fn new(x: char, y:char) -> Self {
        Self {
            x,
            y
        }
    }
}
