// https://atcoder.jp/contests/abc225/tasks/abc225_d

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut trains = vec![(INF, INF); N];

    for _ in 0..Q {
        input!{q: usize}
        if q == 1 {
            input! {
                x: Usize1,
                y: Usize1
            }
            trains[x].1 = y;
            trains[y].0 = x;
        }
        else if q == 2 {
            input! {
                x: Usize1,
                y: Usize1
            }
            trains[x].1 = INF;
            trains[y].0 = INF;
        }
        else if q == 3 {
            input! {
                x: Usize1,
            }
            let mut ans = VecDeque::new();
            ans.push_back(x + 1);

            // 前部を列挙
            let mut i = x;
            while trains[i].0 != INF {
                ans.push_front(trains[i].0 + 1);
                i = trains[i].0;
            }

            // 後部を列挙
            let mut i = x;
            while trains[i].1 != INF {
                ans.push_back(trains[i].1 + 1);
                i = trains[i].1;
            }

            print!("{} ", ans.len());
            println!("{}", ans.iter().join(" "));
        }
    }
}
