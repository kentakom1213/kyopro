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
        N: usize,
        K: usize,
        mut S: [usize; N]
    }

    if S.iter().any(|&x| x == 0) {
        println!("{N}");
        return;
    }

    // 尺取り法
    S.reverse();
    let mut deq = VecDeque::new();
    let mut prod = 1_usize;

    let mut ans = 0;

    for _ in 0..N {
        while let Some(&x) = S.last() {
            if prod.saturating_mul(x) <= K {
                S.pop();
                prod *= x;
                deq.push_back(x);
            } else {
                break;
            }
        }

        // 最大の長さを更新
        debug!(deq);
        ans = ans.max(deq.len());

        // 削除
        if let Some(l) = deq.pop_front() {
            prod /= l;
        }
    }

    println!("{ans}");
}
