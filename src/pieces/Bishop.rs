use crate::util::color::Color;
use crate::util::position::Position;

pub(crate) struct Bishop {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}

impl Bishop {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}