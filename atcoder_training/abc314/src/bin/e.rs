#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut C = vec![0; N];
    let mut P = vec![0; N];
    // K[i] := Sij = 0 になるようなjの数
    let mut K = vec![0; N];
    let mut S = vec![vec![]; N];

    for i in 0..N {
        input! {
            c: usize,
            p: usize,
            s: [usize; p],
        }
        C[i] = c;
        P[i] = p;
        for j in 0..p {
            if s[j] == 0 {
                K[i] += 1;
            } else {
                S[i].push(s[j]);
            }
        }
    }

    // メモ化再帰
    let mut memo = vec![None; M + 1];

    let ans = E(M, &C, &P, &S, &K, &mut memo);

    println!("{ans:.10}");
}

fn E(
    x: usize,
    C: &[usize],
    P: &[usize],
    S: &Vec<Vec<usize>>,
    K: &[usize],
    memo: &mut [Option<f64>],
) -> f64 {
    if x == 0 {
        return 0.0;
    }
    if let Some(v) = memo[x] {
        return v;
    }
    // 期待値最小のサイコロを選択
    let mut res = 1e20;
    for i in 0..S.len() {
        let sum: f64 = S[i]
            .iter()
            .map(|&s| E(x.saturating_sub(s), C, P, S, K, memo))
            .sum();
        // i番目を選んだときの期待値
        let exp = ((P[i] * C[i]) as f64 + sum) / (P[i] - K[i]) as f64;
        if res > exp {
            res = exp;
        }
    }
    memo[x] = Some(res);
    res
}

const INF: usize = 1001001001001001001;
