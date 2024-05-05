#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        S: Chars
    }

    let mut ans = VecDeque::new();
    parser(&S, 0, 0, &mut ans);

    for &c in &ans {
        print!("{c}");
    }
    println!();
}

/// S; 文字列
/// i: 開始位置
fn parser(S: &[char], mut i: usize, layer: usize, res: &mut VecDeque<char>) {
    if i == S.len() {
        return;
    }
    debug!(&S[i..], layer, res);

    let mut tmp = VecDeque::new();

    while i < S.len() {
        match S[i] {
            '(' => {
                // モードを変更
                parser(S, i + 1, layer + 1, res);
                break;
            }
            ')' => {
                // モードを変更
                parser(S, i + 1, layer - 1, res);
                break;
            }
            c => {
                if layer & 1 == 1 {
                    tmp.push_front(toggle(c));
                } else {
                    tmp.push_back(c);
                }
            }
        }
        i += 1;
    }

    if layer & 1 == 1 {
        for &c in &tmp {
            res.push_back(c);
        }
    } else {
        for &c in tmp.iter().rev() {
            res.push_front(c);
        }
    }
}

fn toggle(c: char) -> char {
    if c.is_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}
