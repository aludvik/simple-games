#[macro_use]
extern crate rpg_engine;

mod syllable_sets;


use std::{thread, time};

use rpg_engine::generate::name::NameGenerator;
use rpg_engine::io::screen::Screen;

use syllable_sets::simple_fantasy;

fn main() {
    let syllables = simple_fantasy();
    let ng = NameGenerator::new(&syllables);
    let name = ng.generate(3, 7);
    println!("{}{}", &name[0..1].to_uppercase(), &name[1..]);

    let screen = Screen::new();
    let win = screen.window(0, 0, 10, 3);
    loop {
        match screen.poll() {
            Some(ch) => {
                win.putc(ch);
            },
            None => {
                win.putc('.' as u32);
                thread::sleep(time::Duration::from_millis(100));
            }
        };
        win.refresh();
    }
}
