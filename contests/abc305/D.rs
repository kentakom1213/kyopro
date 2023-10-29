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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
        LR: [(usize, usize); Q],
    }

    // 睡眠時間
    let mut sleep = vec![];
    for i in 1..N {
        if i % 2 == 0 {
            sleep.push(A[i] - A[i - 1]);
        } else {
            sleep.push(0);
        }
    }

    // 累積和
    let mut S = vec![0; N + 1];
    for i in 1..N {
        S[i] = S[i - 1] + sleep[i - 1];
    }

    // クエリの処理
    for &(l, r) in &LR {
        let ll = A.lower_bound(&l);
        let rr = A.lower_bound(&r);
        let mut ans = S[rr] - S[ll];

        // 左のあまりをたす
        if ll % 2 == 0 {
            ans += A[ll] - l;
        }

        // 右のあまりを引く
        if rr % 2 == 0 {
            ans -= A[rr] - r;
        }
        println!("{}", ans);
    }
}
