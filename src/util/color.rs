#[derive(Debug)]
pub(crate) struct Color {
    pub(crate) white: bool,
}

impl Color {
    pub(crate) fn white() -> Self {
        Self {
            white: true,
        }
    }

    pub(crate) fn black() -> Self {
        Self {
            white: false,
        }
    }

    pub(crate) fn is_white(&self) -> bool {
        self.white
    }

    pub(crate) fn is_black(&self) -> bool {
        !self.white
    }
}
