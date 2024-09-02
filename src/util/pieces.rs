use std::any::Any;
use crate::util::color::Color;
use crate::util::position::Position;


pub(crate) trait Piece: Any {
    fn as_any(&self) -> &dyn Any;
    fn color(&self) -> &Color;
    fn position(&self) -> &Position;
    fn has_moved(&self) -> bool;
    fn set_position(&mut self, position: Position);
    fn set_has_moved(&mut self, has_moved: bool);
}

pub(crate) struct Rook {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}
pub(crate) struct Knight {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}
pub(crate) struct Bishop {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}
pub(crate) struct King {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}
pub(crate) struct Queen {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}
pub(crate) struct Pawn {
    pub(crate) color: Color,
    pub(crate) position: Position,
    pub(crate) has_moved: bool,
}


impl Piece for Rook {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
impl Piece for Knight {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
impl Piece for Bishop {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
impl Piece for Queen {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
impl Piece for King {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
impl Piece for Pawn {
    fn as_any(&self) -> &dyn Any { self }

    fn color(&self) -> &Color {
        &self.color
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}

impl Pawn {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}
impl Rook {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}
impl Knight {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
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
impl Queen {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}
impl King {
    pub(crate) fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }
}