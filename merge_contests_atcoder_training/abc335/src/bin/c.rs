// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut deq = (1..=N).map(|x| (x as i32, 0)).collect::<VecDeque<_>>();

    for _ in 0..Q {
        input! {
            t: usize
        }

        if t == 1 {
            // 末尾を先頭に移動
            input! {
                C: char,
            }

            deq.pop_back();
            match C {
                'R' => {
                    let &(r, c) = deq.front().unwrap();
                    deq.push_front((r + 1, c));
                }
                'L' => {
                    let &(r, c) = deq.front().unwrap();
                    deq.push_front((r - 1, c));
                }
                'U' => {
                    let &(r, c) = deq.front().unwrap();
                    deq.push_front((r, c + 1));
                }
                'D' => {
                    let &(r, c) = deq.front().unwrap();
                    deq.push_front((r, c - 1));
                }
                _ => ()
            }
        } else {
            input! {
                q: Usize1,
            }
            let &(r, c) = deq.get(q).unwrap();
            println!("{} {}", r, c);
        }
    }
}
