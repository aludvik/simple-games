use ncurses;

pub struct Window {
    window: ncurses::WINDOW,
}

impl Window {
    pub fn new(window: ncurses::WINDOW) -> Self {
        Window{ window }
    }

    pub fn putc(&self, ch: u32) {
        ncurses::waddch(self.window, ch);
    }

    pub fn refresh(&self) {
        ncurses::wrefresh(self.window);
    }
}
