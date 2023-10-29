//        E - Transformable Teacher        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc181/tasks/abc181_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use superslice::Ext;

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
fn main() {
    input! {
        N: usize,
        M: usize,
        mut H: [usize; N],
        mut W: [usize; M],
    }

    H.sort();
    W.sort();

    // 差分の配列
    let mut D0 = vec![0; N / 2];
    let mut D1 = vec![0; N / 2];

    for i in 0..N / 2 {
        D0[i] = H[2 * i + 1] - H[2 * i];
        D1[i] = H[2 * i + 2] - H[2 * i + 1];
    }

    debug!(&D0);
    debug!(&D1);

    // それぞれの累積和
    let mut S0 = vec![0; N / 2 + 1];
    let mut S1 = vec![0; N / 2 + 1];

    for i in 0..N / 2 {
        S0[i + 1] = S0[i] + D0[i];
        S1[i + 1] = S1[i] + D1[i];
    }

    debug!(&S0);
    debug!(&S1);

    // 全探索
    let mut ans = INF;
    for i in 0..N {
        // i番目の児童以外の合計
        let sum = if i % 2 == 0 {
            S0[i / 2] + (S1[N / 2] - S1[i / 2])
        } else {
            S0[i / 2] + (S1[N / 2] - S1[i / 2 + 1]) + H[i + 1] - H[i - 1]
        };
        debug!(sum);
        // 先生と児童のペアで差が最小なもの
        let mut mini = INF;
        let r = W.lower_bound(&H[i]);
        if r < M {
            mini = mini.min(W[r] - H[i]);
        }
        if r > 0 {
            mini = mini.min(H[i] - W[r - 1]);
        }
        debug!(mini);
        // 答えとなるもの
        ans = ans.min(sum + mini);
    }

    println!("{}", ans);
}
