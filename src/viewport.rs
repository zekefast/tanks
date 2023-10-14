pub(crate) struct Viewport {
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl Viewport {
    fn new(width: u16, height: u16) -> Viewport {
        Viewport {
            width,
            height,
        }
    }

    pub(crate) fn from_terminal() -> Viewport {
        let (width, height) = termion::terminal_size()
            .expect("Can't read terminal size.");

        Viewport::new(width, height)
    }
}
