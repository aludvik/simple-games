use rpg_engine::generate::name::*;

pub fn simple_fantasy() -> SyllableSet {
    SyllableSet{
        vowels: vec![
            syl!("a",  Beginning | Middle | End, 12),
            syl!("e",  Beginning | Middle | End, 12),
            syl!("i",  Beginning | Middle | End, 12),
            syl!("o",  Beginning | Middle | End, 12),
            syl!("u",  Beginning | Middle | End, 12),
            syl!("ae", Beginning | Middle | End, 1),
            syl!("ai", Beginning | Middle | End, 1),
            syl!("ao", Beginning | Middle | End, 1),
            syl!("au", Beginning | Middle | End, 1),
            syl!("aa", Beginning | Middle | End, 1),
            syl!("ea", Beginning | Middle | End, 1),
            syl!("eo", Beginning | Middle | End, 1),
            syl!("eu", Beginning | Middle | End, 1),
            syl!("ee", Beginning | Middle | End, 1),
            syl!("ia", Beginning | Middle | End, 1),
            syl!("io", Beginning | Middle | End, 1),
            syl!("iu", Beginning | Middle | End, 1),
            syl!("ii", Beginning | Middle | End, 1),
            syl!("oa", Beginning | Middle | End, 1),
            syl!("oe", Beginning | Middle | End, 1),
            syl!("oi", Beginning | Middle | End, 1),
            syl!("ou", Beginning | Middle | End, 1),
            syl!("oo", Beginning | Middle | End, 1),
            syl!("'",  Middle, 1),
            syl!("y",  Beginning | Middle | End, 1),
        ],
        consonants: vec![
            syl!("b", Beginning | Middle | End, 3),
            syl!("c", Beginning | Middle | End, 3),
            syl!("d", Beginning | Middle | End, 3),
            syl!("f", Beginning | Middle | End, 3),
            syl!("g", Beginning | Middle | End, 3),
            syl!("h", Beginning | Middle | End, 3),
            syl!("j", Beginning | Middle | End, 3),
            syl!("k", Beginning | Middle | End, 3),
            syl!("l", Beginning | Middle | End, 3),
            syl!("m", Beginning | Middle | End, 3),
            syl!("n", Beginning | Middle | End, 3),
            syl!("p", Beginning | Middle | End, 3),
            syl!("qu", Beginning | Middle, 3),
            syl!("r", Beginning | Middle | End, 3),
            syl!("s", Beginning | Middle | End, 3),
            syl!("t", Beginning | Middle | End, 3),
            syl!("v", Beginning | Middle | End, 3),
            syl!("w", Beginning | Middle | End, 3),
            syl!("x", Beginning | Middle | End, 1),
            syl!("y", Beginning | Middle | End, 1),
            syl!("z", Beginning | Middle | End, 1),
            syl!("sc", Beginning | Middle | End, 1),
            syl!("ch", Beginning | Middle | End, 1),
            syl!("gh", Beginning | Middle | End, 1),
            syl!("ph", Beginning | Middle | End, 1),
            syl!("sh", Beginning | Middle | End, 1),
            syl!("th", Beginning | Middle | End, 1),
            syl!("wh", Beginning | Middle, 1),
            syl!("ck", End | Middle, 1),
            syl!("nk", End | Middle, 1),
            syl!("rk", End | Middle, 1),
            syl!("sk", Beginning | Middle | End, 1),
            syl!("wk", Middle, 1),
            syl!("cl", Beginning | Middle, 1),
            syl!("fl", Beginning | Middle, 1),
            syl!("gl", Beginning | Middle, 1),
            syl!("kl", Beginning | Middle, 1),
            syl!("ll", Beginning | Middle, 1),
            syl!("pl", Beginning | Middle, 1),
            syl!("sl", Beginning | Middle, 1),
            syl!("br", Beginning | Middle, 2),
            syl!("cr", Beginning | Middle, 1),
            syl!("dr", Beginning | Middle, 2),
            syl!("fr", Beginning | Middle, 2),
            syl!("gr", Beginning | Middle, 2),
            syl!("kr", Beginning | Middle, 2),
            syl!("pr", Beginning | Middle, 1),
            syl!("sr", Beginning | Middle, 1),
            syl!("tr", Beginning | Middle, 1),
            syl!("ss", End | Middle, 1),
            syl!("st", Beginning | Middle | End, 1),
            syl!("str", Beginning | Middle, 1),
        ],
    }
}