#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        S: String
    }

    let toggle = |c: char| -> char {
        if c.is_uppercase() {
            c.to_ascii_lowercase()
        } else {
            c.to_ascii_uppercase()
        }
    };

    // 予め、大文字小文字を修正
    let mut T = vec![];
    // 対応するカッコのペア
    let mut pair = FxHashMap::default();
    let mut st = vec![];
    let mut is_on = false;

    for (i, c) in S.chars().enumerate() {
        match c {
            '(' => {
                st.push(i);
                T.push(c);
                is_on = !is_on;
            }
            ')' => {
                let left = st.pop().unwrap();
                pair.insert(left, i);
                pair.insert(i, left);
                T.push(c);
                is_on = !is_on;
            }
            _ => {
                if is_on {
                    T.push(toggle(c));
                } else {
                    T.push(c);
                }
            }
        }
    }

    debug!(T);
    debug!(pair);

    // 以下、反転処理のみ考えれば良い
    parser(&T, 0, T.len() - 1, 0, &pair);
    println!();
}

/// S[l..=r] を処理（閉区間）
fn parser(S: &[char], l: usize, r: usize, depth: usize, pair: &FxHashMap<usize, usize>) {
    debug!(&S[l..=r], depth);
    if l > r {
        return;
    }

    if depth & 1 == 0 {
        let mut i = l;
        while i <= r {
            if S[i] == '(' {
                let close = pair[&i];
                parser(S, i + 1, close - 1, depth + 1, pair);
                i = close;
            } else {
                print!("{}", S[i]);
            }
            i += 1;
        }
    } else {
        let mut i = r;
        while i >= l {
            if S[i] == ')' {
                let open = pair[&i];
                parser(S, open + 1, i - 1, depth + 1, pair);
                i = open;
            } else {
                print!("{}", S[i]);
            }
            i -= 1;
        }
    }
}
