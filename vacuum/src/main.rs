extern crate engine;

use std::{thread, time};

use engine::io::screen::Screen;
use engine::io::window::Window;
use engine::io::keys::*;
use engine::world::scene::Scene;

type Vec2 = (i32, i32);

struct GameState {
    pub vacuum: Vec2,
    pub dust: Vec<Vec2>,
    pub score: i32,
}

fn main() {
    let (w, h) = (32, 24);
    let mut scene = MainWindow{
        state: GameState{
            vacuum: (w / 2, h / 2),
            dust: vec![(2, 2), (14, 8), (28, 22)],
            score: 0,
        },
        max: (w - 1, h - 1),
    };
    let screen = Screen::new();
    let mut win = screen.window(0, 0, w as u32, h as u32);
    loop {
        match screen.poll() {
            Some(key) => {
                scene.on_pressed(key as i32);
            },
            None => {
                thread::sleep(time::Duration::from_millis(10));
            }
        };
        win.clear();
        scene.draw(&mut win);
        win.refresh();
    }
}

struct MainWindow {
    state: GameState,
    max: Vec2,
}

impl MainWindow {
    fn draw_at(&self, c: char, at: Vec2, window: &Window) {
        let (x, y) = at;
        window.mvputc(x as u32, y as u32, c);
    }

    fn draw_border(&self, window: &Window) {
        let (mx, my) = self.max;
        for x in 0..mx {
            window.mvputc(x as u32, 0 as u32, '-');
            window.mvputc(x as u32, my as u32, '-');
        }
        for y in 0..my {
            window.mvputc(0 as u32, y as u32, '|');
            window.mvputc(mx as u32, y as u32, '|');
        }
        window.mvputc(0 as u32, 0 as u32, '+');
        window.mvputc(mx as u32, 0 as u32, '+');
        window.mvputc(0 as u32, my as u32, '+');
        window.mvputc(mx as u32, my as u32, '+');
    }
}

fn wrap(cur: i32, del: i32, min: i32, max: i32) -> i32 {
    match cur + del {
        x if x > max => max,
        x if x < min => min,
        x => x,
    }
}

fn wrap2(
    cur: (i32, i32),
    del: (i32, i32),
    min: (i32, i32),
    max: (i32, i32),
) -> (i32, i32) {
    (
        wrap(cur.0, del.0, min.0 + 1, max.0 - 1),
        wrap(cur.1, del.1, min.1 + 1, max.1 - 1),
    )
}

impl Scene for MainWindow {
    fn draw(&self, window: &Window) {
        self.draw_border(window);
        for dust in &self.state.dust {
            self.draw_at('*', *dust, window);
        }
        self.draw_at('[', self.state.vacuum, window);
    }

    fn update(&mut self) {
    }

    fn on_pressed(&mut self, key: Key) {
        let dp: (i32, i32) = match key {
            LOWER_A | LEFT => (-1, 0),
            LOWER_W | UP => (0, -1),
            LOWER_D | RIGHT => (1, 0),
            LOWER_S | DOWN => (0, 1),
            _ => (0, 0),
        };
        self.state.vacuum = wrap2(
            self.state.vacuum,
            dp, (0, 0), self.max,
        );
    }
}
