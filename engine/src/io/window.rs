use ncurses;

pub struct Window {
    window: ncurses::WINDOW,
}

impl Window {
    pub fn new(window: ncurses::WINDOW) -> Self {
        Window{ window }
    }

    pub fn putc(&self, ch: char) {
        ncurses::waddch(self.window, ch as u32);
    }

    pub fn refresh(&self) {
        ncurses::wrefresh(self.window);
    }
}
