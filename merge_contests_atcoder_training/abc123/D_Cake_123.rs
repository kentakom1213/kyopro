//               D - Cake 123
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc123/tasks/abc123_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
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
const NEG1: usize = 1_usize.wrapping_neg();

fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        K: usize,
        A: [usize; X],
        B: [usize; Y],
        C: [usize; Z],
    }

    // A, Bの直積
    let AxB: Vec<usize> = iproduct!(A.iter(), B.iter())
        .map(|(&a, &b)| a + b)
        .sorted()
        .collect();

    debug!(&AxB);

    // AxBxCのそれぞれの総和で値がx以上のものの個数
    let count_lb = |x: usize| -> usize {
        let mut cnt = 0_usize;
        // Cに関して全探索
        for &c in &C {
            let val = x.saturating_sub(c);
            let mut ng = NEG1;
            let mut ok = X * Y;
            while ok.wrapping_sub(ng) > 1 {
                let mid = ok.wrapping_add(ng) / 2;
                if AxB[mid] >= val {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            cnt += X * Y - ok;
        }
        cnt
    };

    // K個取り出す
    for i in 0..K {
        // 上位idx個目より真に大きい値を探索
        let mut ok = 0;
        let mut ng = INF;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if count_lb(mid) > i {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}
