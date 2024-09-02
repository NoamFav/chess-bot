#[derive(Debug)]
pub(crate) struct Position {
    position: String,
}

impl Position {
    pub(crate) fn new(position: String) -> Self {
        Self {
            position,
        }
    }
}