use rand::{thread_rng, Rng};
use rand::distributions::{IndependentSample, Range};
use std::iter::FromIterator;

pub struct SyllableSet {
    pub consonants: Vec<Syllable>,
    pub vowels: Vec<Syllable>,
}

#[derive(PartialEq)]
pub struct Syllable {
    pub characters: &'static str,
    pub positions: Vec<Position>,
    pub weight: u16,
}

#[derive(PartialEq)]
pub enum Position { Beginning, Middle, End }

#[macro_export]
macro_rules! syl {
    ($s:expr, $($p:ident)|+, $w:expr) => {
        Syllable{
            characters: $s,
            positions: vec![$(Position::$p),+],
            weight: $w,
        }
    }
}

pub struct NameGenerator<'s> {
    // consonants
    beg_cons: Vec<&'s Syllable>,
    beg_cons_w: u16,
    mid_cons: Vec<&'s Syllable>,
    mid_cons_w: u16,
    end_cons: Vec<&'s Syllable>,
    end_cons_w: u16,
    // vowels
    beg_vows: Vec<&'s Syllable>,
    beg_vows_w: u16,
    mid_vows: Vec<&'s Syllable>,
    mid_vows_w: u16,
    end_vows: Vec<&'s Syllable>,
    end_vows_w: u16,
}

impl<'s> NameGenerator<'s> {
    pub fn new(set: &'s SyllableSet) -> Self {
        // Filter into groups and cache for faster generation
        let (beg_cons, beg_cons_w) =
            Self::filter_and_weight(
                &set.consonants, Position::Beginning);
        let (mid_cons, mid_cons_w) =
            Self::filter_and_weight(
                &set.consonants, Position::Middle);
        let (end_cons, end_cons_w) =
            Self::filter_and_weight(
                &set.consonants, Position::End);
        let (beg_vows, beg_vows_w) =
            Self::filter_and_weight(
                &set.vowels, Position::Beginning);
        let (mid_vows, mid_vows_w) =
            Self::filter_and_weight(
                &set.vowels, Position::Middle);
        let (end_vows, end_vows_w) =
            Self::filter_and_weight(
                &set.vowels, Position::End);

        NameGenerator {
            beg_cons, beg_cons_w,
            mid_cons, mid_cons_w,
            end_cons, end_cons_w,
            beg_vows, beg_vows_w,
            mid_vows, mid_vows_w,
            end_vows, end_vows_w,
        }
    }

    // If min > max or min < 2, returns an empty string. Otherwise, generates a random name.
    pub fn generate(&self, min_syllables: u16, max_syllables: u16) -> String {
        if max_syllables < min_syllables {
            String::from("")
        } else if min_syllables < 2 {
            String::from("")
        } else {
            let length = {
                if min_syllables == max_syllables {
                    min_syllables
                } else {
                    Self::rand_range(min_syllables, max_syllables)
                }
            };
            let mut name = Vec::with_capacity(length as usize);

            let mut vowel = Self::flip();
            name.push({
                if vowel {
                    Self::rand_syllable(&self.beg_cons, self.beg_cons_w)
                } else {
                    Self::rand_syllable(&self.beg_vows, self.beg_vows_w)
                }
            });
            vowel = !vowel;

            for _ in 0..(length - 2) {
                name.push({
                    if vowel {
                        Self::rand_syllable(&self.mid_cons, self.mid_cons_w)
                    } else {
                        Self::rand_syllable(&self.mid_vows, self.mid_vows_w)
                    }
                });
                vowel = !vowel;
            }

            name.push({
                if vowel {
                    Self::rand_syllable(&self.end_cons, self.end_cons_w)
                } else {
                    Self::rand_syllable(&self.end_vows, self.end_vows_w)
                }
            });

            String::from_iter(name.into_iter())
        }
    }

    fn filter_and_weight(syllables: &[Syllable], pos: Position) -> (Vec<&Syllable>, u16) {
        let filtered: Vec<&Syllable> = syllables.iter().filter(|s|
            s.positions.iter().any(|p|
                p == &pos)).collect();
        let total_weight = filtered.iter().fold(0, |t, s| t + s.weight);
        (filtered, total_weight)
    }

    fn rand_syllable(syllables: &[&Syllable], total_weight: u16) -> String {
        let mut weight = Self::rand_range(0, total_weight);
        for syllable in syllables {
            if weight < syllable.weight {
                return String::from(syllable.characters);
            }
            weight -= syllable.weight
        }
        match syllables.last() {
            Some(syllable) => String::from(syllable.characters),
            None => String::from(""),
        }
    }

    fn rand_range(min: u16, max: u16) -> u16 {
        Range::new(min, max).ind_sample(&mut thread_rng())
    }

    fn flip() -> bool { thread_rng().gen() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_and_weight() {
        let syllables: Vec<Syllable> = vec![
            syl!("a", Beginning, 5),
            syl!("b", Middle, 2),
            syl!("c", Beginning, 2)
        ];

        let (filtered, weight): (Vec<&Syllable>, u16) =
            NameGenerator::filter_and_weight(&syllables, Position::Beginning);
        assert!(weight == 7);
        assert!(filtered[0] == syllables.iter().collect::<Vec<&Syllable>>()[0])
    }

    #[test]
    fn test_flip() {
        let b = NameGenerator::flip();
        assert!(b || !b);
    }

    #[test]
    fn test_rand_range() {
        let n = NameGenerator::rand_range(0, 2);
        assert!(n == 2 || n == 1 || n == 0);
    }

    #[test]
    fn test_rand_syllable() {
        let syllables: Vec<Syllable> = vec![
            syl!("a", Beginning, 5),
            syl!("b", Middle, 2),
        ];
        let borrowed: Vec<&Syllable> = syllables.iter().collect();
        let s = &NameGenerator::rand_syllable(&borrowed, 7);
        assert!(&s[..] == "a" || &s[..] == "b");
    }

    #[test]
    fn test_name_generator() {
        let set = SyllableSet{
            vowels: vec![
                syl!("a", Beginning | Middle, 1),
                syl!("e", Middle, 2),
                syl!("i", End, 3),
                syl!("o", Beginning | Middle | End, 4),
                syl!("u", Beginning | Middle | End, 4),
            ],
            consonants: vec!(
                syl!("ch", Beginning | Middle | End, 8),
                syl!("t", Middle, 6),
                syl!("s", Beginning, 5),
                syl!("l", End, 7),
            ),
        };
        let ng = NameGenerator::new(&set);
        assert!(&ng.generate(1, 3)[..] == "");
        assert!(&ng.generate(5, 2)[..] == "");
        assert!(&ng.generate(2, 5)[..] != "");
    }
}
