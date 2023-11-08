// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
        S: usize,
        card: [(usize, usize); N]
    }

    // dp
    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;

    for i in 0..N {
        for j in 0..=S {
            if !dp[i][j] {
                continue;
            }
            if j + card[i].0 <= S {
                dp[i + 1][j + card[i].0] = true;
            }
            if j + card[i].1 <= S {
                dp[i + 1][j + card[i].1] = true;
            }
        }
    }

    // 判定
    if !dp[N][S] {
        println!("No");
        return;
    }

    println!("Yes");

    // 復元
    let mut cur = S;
    let mut res = vec![];
    for i in (1..=N).rev() {
        if cur >= card[i - 1].0 && dp[i - 1][cur - card[i - 1].0] {
            res.push('H');
            cur -= card[i - 1].0;
        } else {
            res.push('T');
            cur -= card[i - 1].1;
        }
    }

    res.reverse();
    println!("{}", res.iter().join(""));
}
