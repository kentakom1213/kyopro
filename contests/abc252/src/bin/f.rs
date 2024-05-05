// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BinaryHeap};

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        L: usize,
        A: [usize; N]
    }

    // ヒープで小さい要素から貪欲にマージしていく
    let mut pq: BinaryHeap<Reverse<usize>> = A.iter().map(|&v| Reverse(v)).collect();

    let sum = A.iter().sum::<usize>();

    if L > sum {
        pq.push(Reverse(L - sum));
    }

    let mut ans = 0;

    while pq.len() >= 2 {
        let fst = pq.pop().unwrap().0;
        let snd = pq.pop().unwrap().0;

        ans += fst + snd;

        pq.push(Reverse(fst + snd));
    }

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
