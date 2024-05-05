// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::{sorted, Itertools};
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        mut DCS: [(isize, isize, usize); N]
    }

    // 終了時刻でソート
    DCS.sort();

    let mut ans = 0;

    // 全探索
    for i in 0..1 << N {
        // 現在時刻
        let mut t = 1;
        // スコア
        let mut score = 0;

        for j in (0..N).filter(|j| i >> j & 1 == 1) {
            let (d, c, s) = DCS[j];
            if t + c <= d {
                score += s;
                t += c;
            }
        }

        ans = ans.max(score);
    }

    println!("{ans}");
}
