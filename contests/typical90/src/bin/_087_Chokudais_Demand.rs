//       087 - Chok_bottomudai's Demand（★5）       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_ci
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Usize1}};

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
        P: usize,
        K: usize,
        A: [[isize; N]; N]
    }

    debug!(floyd_warshall_cnt(&A, INF, P));

    // 上界が存在しないとき
    if floyd_warshall_cnt(&A, INF, P) == K {
        println!("Infinity");
        return;
    }

    // 2分探索で上界を求める
    let mut ok_top = 0;
    let mut ng_top = INF;
    while ng_top - ok_top > 1 {
        let mid = (ok_top + ng_top) / 2;
        // INFをmidに置き換え
        let cnt = floyd_warshall_cnt(&A, mid, P);
        // 境界を更新
        if cnt >= K {
            ok_top = mid;
        } else {
            ng_top = mid;
        }
    }

    debug!(ok_top, ng_top);

    let mut ok_bottom = 0;
    let mut ng_botton = INF;
    while ng_botton - ok_bottom > 1 {
        let mid = (ok_bottom + ng_botton) / 2;
        // INFをmidに置き換え
        let cnt = floyd_warshall_cnt(&A, mid, P);
        // 境界を更新
        if cnt > K {
            ok_bottom = mid;
        } else {
            ng_botton = mid;
        }
    }

    debug!(ok_bottom, ng_botton);

    println!("{}", ok_top - ok_bottom);
}

fn floyd_warshall_cnt(A: &Vec<Vec<isize>>, x: usize, P: usize) -> usize {
    let n = A.len();
    let mut res = vec![vec![0; n]; n];
    // INFをxにおきかえ
    for i in 0..n {
        for j in 0..n {
            res[i][j] = if A[i][j] == -1 {
                x
            } else {
                A[i][j] as usize
            };
        }
    }
    // ワーシャルフロイド法
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                res[i][j] = res[i][j].min(
                    res[i][k] + res[k][j]
                );
            }
        }
    }
    // P以下の経路の個数を調べる
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..i {
            if res[i][j] <= P {
                cnt += 1;
            }
        }
    }
    cnt
}
