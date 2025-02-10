#![allow(dead_code)]

const ALLOWED_STR: &[u8] = include_bytes!("../new-allowed.txt");
const POSSIBLE_STR: &[u8] = include_bytes!("../adjusted-possible-2025-02-10.txt");

const ALLOWED_WORDS_LEN: usize = {
    let mut n = 0;
    let mut i = 0;
    while i < ALLOWED_STR.len() {
        if ALLOWED_STR[i] == b'\n' {
            n += 1;
        }
        i += 1;
    }
    n
};
pub const POSSIBLE_WORDS_LEN: usize = {
    let mut n = 0;
    let mut i = 0;
    while i < POSSIBLE_STR.len() {
        if POSSIBLE_STR[i] == b'\n' {
            n += 1;
        }
        i += 1;
    }
    n
};
const COMBINED_LEN: usize = ALLOWED_WORDS_LEN + POSSIBLE_WORDS_LEN;

pub type Word = [u8; 5];

pub const fn word_to_string(word: &Word) -> &str {
    unsafe { std::str::from_raw_parts(word.as_ptr(), 5) }
}

pub const ALLOWED_WORDS: [Word; ALLOWED_WORDS_LEN] = {
    let mut words = [[0; 5]; ALLOWED_WORDS_LEN];
    let mut i = 0;
    while i < ALLOWED_WORDS_LEN {
        let j = i * 6;
        let mut k = 0;
        while k < 5 {
            words[i][k] = ALLOWED_STR[j + k];
            k += 1;
        }
        i += 1;
    }
    words
};
pub const POSSIBLE_WORDS: [Word; POSSIBLE_WORDS_LEN] = {
    let mut words = [[0; 5]; POSSIBLE_WORDS_LEN];
    let mut i = 0;
    while i < POSSIBLE_WORDS_LEN {
        let j = i * 6;
        let mut k = 0;
        while k < 5 {
            words[i][k] = POSSIBLE_STR[j + k];
            k += 1;
        }
        i += 1;
    }
    words
};

pub const COMBINED_WORDS: [Word; COMBINED_LEN] = {
    let mut out = [[0; 5]; COMBINED_LEN];
    let mut i = 0;
    while i < ALLOWED_WORDS.len() {
        out[i] = ALLOWED_WORDS[i];
        i += 1;
    }
    let mut j = 0;
    while j < POSSIBLE_WORDS.len() {
        out[i] = POSSIBLE_WORDS[j];
        i += 1;
        j += 1;
    }
    out
};

const fn binomial_coeff(n: usize, k: usize) -> usize {
    if k >= n {
        1
    } else if k == 1 {
        n
    } else {
        let mut out = 1;
        let mut a = n - k + 1;
        let mut b = 2;
        loop {
            if b <= k && out % b == 0 {
                out /= b;
                b += 1;
            } else if a % b == 0 {
                out *= a / b;
                a += 1;
                b += 1;
            } else if a <= n {
                out *= a;
                a += 1;
            } else {
                break;
            }
        }
        out
    }
}

pub const TOTAL_STRATS: usize = binomial_coeff(COMBINED_LEN, crate::DEPTH);
pub const TOTAL_STRATS_LEN: usize = {
    let mut ndigits = 0;
    let mut acc = TOTAL_STRATS;
    while acc > 0 {
        let cur_digit = acc % 10;
        acc -= cur_digit;
        acc /= 10;
        ndigits += 1;
    }
    ndigits
};
