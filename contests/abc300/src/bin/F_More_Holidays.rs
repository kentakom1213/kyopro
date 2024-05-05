//            F - More Holidays
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc300/tasks/abc300_f
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
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        S: String,
    }

    // Sに含まれる文字xの累積
    let mut ss = vec![0; N + 1];
    for (i, x) in S.chars().enumerate() {
        ss[i + 1] = ss[i] + (x == 'x') as usize;
    }
    debug!(&ss);

    // Tのx文字目までに何個のxが含まれるか
    let f = |x: usize| -> usize {
        let res = (x / N) * ss[N];
        let rem = x % N;
        res + ss[rem]
    };

    // 区間に含まれるxの個数がrem個以下になるような区間の最大の長さ
    let mut ans = 0;
    for i in 1..=N {
        let mut ok = i;
        let mut ng = N * M + 1;
        let f_start = f(i - 1);
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if f(mid) - f_start <= K {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans = ans.max(ok - i + 1);
    }

    println!("{}", ans);
}
