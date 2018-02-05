use ncurses;

use io::window::Window;

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        ncurses::initscr();
        ncurses::cbreak();
        ncurses::nodelay(ncurses::stdscr(), true);
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        Screen{}
    }

    pub fn clear(&self) {
        ncurses::clear();
    }

    pub fn poll(&self) -> Option<u32> {
        match ncurses::getch() {
            i if i < 0 => None,
            i => Some(i as u32),
        }
    }

    pub fn default_window(&self) -> Window {
        Window::new(ncurses::stdscr())
    }

    pub fn window(&self, x: u32, y: u32, w: u32, h: u32) -> Window {
        Window::new(ncurses::newwin(
            h as i32, w as i32, y as i32, x as i32))
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen() {
        let screen = Screen::new();
        screen.poll();
    }
}
