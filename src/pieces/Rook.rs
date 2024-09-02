use crate::util::color::Color;
use crate::util::position::Position;

pub(crate) struct Rook {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}

impl Rook{
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}