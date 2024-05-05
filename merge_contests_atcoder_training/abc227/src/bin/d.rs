#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BinaryHeap;

use proconio::input;

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

fn main() {
    input! {
        N: usize,
        K: i128,
        mut A: [i128; N]
    }

    // m個のグループを作れるか
    let can = |m| -> bool {
        let sum = A.iter().map(|&a| a.min(m)).sum::<i128>();
        sum >= K * m
    };

    // 解の存在で二分探索
    let mut ok = 0;
    let mut ng = INF as i128;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if can(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

const INF: usize = 1001001001001001001;
