#[macro_use]
extern crate rpg_engine;

mod syllable_sets;


use rpg_engine::generate::name::NameGenerator;

use syllable_sets::simple_fantasy;

fn main() {
    let syllables = simple_fantasy();
    let ng = NameGenerator::new(&syllables);
    let name = ng.generate(3, 7);
    println!("{}{}", &name[0..1].to_uppercase(), &name[1..]);
}
