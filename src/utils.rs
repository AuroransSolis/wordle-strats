use crate::words::{word_to_string, Word};
use ordes::OrdesConcat;
use std::simd::{prelude::SimdPartialEq, u8x8};

pub fn strat_strings<const N: usize>(words: &[Word; N]) -> [&str; N] {
    let mut output = [""; N];
    for i in 0..N {
        output[i] = word_to_string(&words[i]);
    }
    output
}

pub struct AnalysisResult {
    pub avg: f64,
    pub std: f64,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Filter {
    Exclude(u8),
    WrongLoc(u8),
    TooMany(u8, u8),
    Correct(u8),
}

impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::Exclude(l) => write!(f, "Exclude({:?})", *l as char),
            Filter::WrongLoc(l) => write!(f, "WrongLoc({:?})", *l as char),
            Filter::TooMany(l, c) => write!(f, "TooMany({:?}, {c})", *l as char),
            Filter::Correct(l) => write!(f, "Correct({:?})", *l as char),
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Exclude(0)
    }
}

pub fn guess_to_filters(secret: Word, guess: Word) -> [Filter; 5] {
    let mut out = guess.map(Filter::Exclude);
    let mut used = [false; 5];
    'outer: for i in 0..5 {
        let letter = guess[i];
        if letter == secret[i] {
            used[i] = true;
            out[i] = Filter::Correct(letter);
        } else {
            let mut count_eq = 0;
            for j in 0..5 {
                if secret[j] == letter {
                    if !used[j] {
                        out[i] = Filter::WrongLoc(letter);
                        used[j] = true;
                        continue 'outer;
                    }
                    count_eq += 1;
                }
            }
            if count_eq > 0 {
                out[i] = Filter::TooMany(letter, count_eq);
            }
        }
    }
    out
}

pub fn word_passes_filters(word: Word, filters: [Filter; 5]) -> bool {
    let wordsimd = u8x8::from_array([0u8; 3].concatenate(word));
    for (i, filter) in filters.into_iter().enumerate() {
        match filter {
            Filter::Exclude(eletter) => {
                let mask = u8x8::splat(eletter);
                let res = wordsimd.simd_eq(mask);
                if res.any() {
                    return false;
                }
            }
            Filter::WrongLoc(letter) => {
                if word[i] == letter {
                    return false;
                }
                let mask = u8x8::from_array([1u8; 3].concatenate([letter; 5]));
                let res = wordsimd.simd_ne(mask);
                if res.all() {
                    return false;
                }
            }
            Filter::TooMany(letter, max) => {
                let mask = u8x8::splat(letter);
                let res = wordsimd.simd_eq(mask).to_array();
                if res.into_iter().filter(|&eq| eq).count() as u8 > max {
                    return false;
                }
            }
            Filter::Correct(cletter) => {
                if word[i] != cletter {
                    return false;
                }
            }
        }
    }
    true
}

pub struct Aggregate {
    count: f64,
    mean: f64,
    m2: f64,
}

impl Aggregate {
    pub fn new() -> Self {
        Aggregate {
            count: 0.0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    pub fn update(&mut self, new: f64) {
        self.count += 1.0;
        let delta = new - self.mean;
        self.mean += delta / self.count;
        let delta2 = new - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn finalise(self) -> [f64; 2] {
        let mean = self.mean;
        let variance = self.m2 / self.count;
        let stddev = variance.sqrt();
        [mean, stddev]
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::{
        guess_to_filters, Aggregate,
        Filter::{self, *},
    };

    #[test]
    fn test_all_correct() {
        let word = [0; 5];
        let guess = word;
        let feedback = guess_to_filters(word, guess);
        assert_eq!(feedback, [Correct(0); 5]);
    }

    #[test]
    fn test_all_exclude() {
        let word = [0; 5];
        let guess = [1; 5];
        let feedback = guess_to_filters(word, guess);
        assert_eq!(feedback, [Exclude(1); 5]);
    }

    #[test]
    fn test_wrong_loc() {
        let word = [1, 0, 0, 0, 0];
        let guess = [0, 1, 0, 0, 0];
        let feedback = guess_to_filters(word, guess);
        assert_eq!(
            feedback,
            [WrongLoc(0), WrongLoc(1), Correct(0), Correct(0), Correct(0)]
        );
    }

    #[test]
    fn test_too_many() {
        let word = [1, 0, 0, 0, 0];
        let guess = [1, 1, 1, 0, 0];
        let feedback = guess_to_filters(word, guess);
        assert_eq!(
            feedback,
            [
                Correct(1),
                TooMany(1, 1),
                TooMany(1, 1),
                Correct(0),
                Correct(0)
            ]
        );
    }

    #[test]
    fn test_agg() {
        let data = [2, 4, 4, 4, 5, 5, 7, 9];
        let count0 = data.len() as f64;
        let mean0 = data.into_iter().map(|d| d as f64).sum::<f64>() / count0;
        let var0 = data
            .into_iter()
            .map(|d| (d as f64 - mean0).powf(2.0))
            .sum::<f64>()
            / count0;
        let std0 = var0.sqrt();
        let data0 = [mean0, std0];
        let mut agg = Aggregate::new();
        data.into_iter().for_each(|d| agg.update(d as f64));
        let data1 = agg.finalise();
        let names = ["mean", "std"];
        for ((a, b), name) in data0
            .into_iter()
            .zip(data1.into_iter())
            .zip(names.into_iter())
        {
            match a.partial_cmp(&b) {
                Some(std::cmp::Ordering::Equal) => continue,
                _ => {
                    println!("comparison '{name}' failed!");
                    println!("2P: {a} ({data0:?})");
                    println!("WF: {b} ({data1:?})");
                    panic!();
                }
            }
        }
    }
}
