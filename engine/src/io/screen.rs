use ncurses;

use io::window::Window;

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        // Setup ncurses
        ncurses::initscr();
        // Catch control characters like CTRL+C
        ncurses::cbreak();
        // Non-blocking getch()
        ncurses::nodelay(ncurses::stdscr(), true);
        // Do not print characters to screen by default
        ncurses::noecho();
        // Do not show the cursor
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        Screen{}
    }

    // Get input
    pub fn poll(&self) -> Option<char> {
        match ncurses::getch() {
            c if c < 0 => None,
            c => Some(((c as u32) as u8) as char),
        }
    }

    // Create windows
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
