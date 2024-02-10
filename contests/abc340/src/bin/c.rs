#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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
    }

    // let mut pq = BinaryHeap::from([N]);
    // let mut ans = 0;

    // while !pq.is_empty() && pq.peek().unwrap() >= &2 {
    //     let x = pq.pop().unwrap();
    //     ans += x;
    //     let floor = x / 2;
    //     let ceil = (x + 1) / 2;
    //     pq.push(floor);
    //     pq.push(ceil);
    // }

    // println!("{ans}");

    // メモ

}


