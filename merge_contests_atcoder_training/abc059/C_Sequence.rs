//               C - Sequence
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc059/tasks/arc072_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        n: usize,
        A: [isize; n],
    }

    let mut ans = INF;

    for f in vec![-1_isize, 1] {
        let mut tmp = 0;
        let mut sum = 0;

        for i in 0..n {
            sum += A[i];
            if i % 2 == 0 {
                if sum * f <= 0 {
                    // 同符号になるように調整
                    tmp += (sum - f).abs();
                    sum = f;
                }
            } else {
                if sum * f >= 0 {
                    // 異符号になるように調整
                    tmp += (sum + f).abs();
                    sum = -f;
                }
            }
            debug!(f, sum, tmp);
        }

        ans = ans.min(tmp);
    }

    println!("{}", ans);
}
