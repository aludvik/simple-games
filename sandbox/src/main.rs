#[macro_use]
extern crate rpg_engine;

mod syllable_sets;


use std::{thread, time};

use rpg_engine::generate::name::NameGenerator;
use rpg_engine::io::screen::Screen;
use rpg_engine::io::window::Window;
use rpg_engine::world::scene::Scene;

use syllable_sets::simple_fantasy;

fn main() {
    //gen_name();
    //poll_win();
    // simple_scene();
    collect_char_nums();
}

struct SimpleScene {
    position: (i32, i32),
    max: (i32, i32),
}

fn wrap2(
    cur: (i32, i32),
    del: (i32, i32),
    min: (i32, i32),
    max: (i32, i32),
) -> (i32, i32) {
    (
        wrap(cur.0, del.0, min.0, max.0),
        wrap(cur.1, del.1, min.1, max.1),
    )
}

fn wrap(cur: i32, del: i32, min: i32, max: i32) -> i32 {
    match cur + del {
        x if x > max => max,
        x if x < min => min,
        x => x,
    }
}

impl Scene for SimpleScene {
    fn draw(&self, window: &Window) {
        let (x, y) = self.position;
        window.mvputc(x as u32, y as u32, '0');
    }

    fn update(&mut self) {
    }

    fn on_pressed(&mut self, key: char) {
        let dp: (i32, i32) = match key {
            'a' => (-1, 0),
            'w' => (0, -1),
            'd' => (1, 0),
            's' => (0, 1),
            _ => (0, 0),
        };
        self.position = wrap2(
            self.position,
            dp,
            (0, 0),
            self.max,
        );
    }
}

fn simple_scene() {
    let (w, h) = (20, 10);
    let mut scene = SimpleScene{
        position: (w / 2, h / 2),
        max: (w - 1, h - 1),
    };
    let screen = Screen::new();
    let mut win = screen.window(0, 0, w as u32, h as u32);
    loop {
        match screen.poll() {
            Some(ch) => {
                scene.on_pressed(ch);
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

fn collect_char_nums() {
    let mut keys: Vec<u32> = Vec::new();
    {
        let screen = Screen::new();
        loop {
            match screen.poll() {
                Some(ch) => match ch {
                    'q' => {
                        break;
                    },
                    _ => {
                        keys.push(ch as u32);
                    }
                },
                None => {
                    thread::sleep(time::Duration::from_millis(10));
                },
            };
        }
    }
    for key in keys {
        println!("{}", key);
    }
}

fn poll_win() {
    let screen = Screen::new();
    let win = screen.window(0, 0, 100, 30);
    loop {
        match screen.poll() {
            Some(ch) => {
                win.putc(ch);
            },
            None => {
                win.putc('.');
                thread::sleep(time::Duration::from_millis(1000));
            }
        };
        win.refresh();
    }
}

fn gen_name() {
    let syllables = simple_fantasy();
    let ng = NameGenerator::new(&syllables);
    let name = ng.generate(3, 7);
    println!("{}{}", &name[0..1].to_uppercase(), &name[1..]);
}
