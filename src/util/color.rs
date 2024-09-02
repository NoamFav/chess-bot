#[derive(Debug)]
pub(crate) struct Color {
    white: bool,
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
}