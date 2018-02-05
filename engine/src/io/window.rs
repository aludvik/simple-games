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

    pub fn mvputc(&self, x: u32, y: u32, ch: char) {
        ncurses::mvwaddch(self.window, y as i32, x as i32, ch as u32);
    }

    pub fn clear(&self) {
        ncurses::wclear(self.window);
    }

    pub fn refresh(&self) {
        ncurses::wrefresh(self.window);
    }
}
