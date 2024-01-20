#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

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
        A: [isize; N]
    }

    // 前後
    let mut head = INF;
    let mut ord = vec![INF; N];

    for (i, &a) in A.iter().enumerate() {
        if a == -1 {
            head = i;
            continue;
        }
        ord[a as usize - 1] = i;
    }

    debug!(ord);

    let mut ans = vec![];
    for _ in 0..N {
        ans.push(head + 1);
        head = ord[head];
    }

    println!("{}", ans.iter().join(" "));
}
