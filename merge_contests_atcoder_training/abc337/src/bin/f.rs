#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

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

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        C: [Usize1; N]
    }

    let mut boxid = vec![INF; N];
    let mut boxes = vec![0_usize; M];

    // 愚直解
    for i in 0..N {
        let mut used = 0;
        let mut cnt = 0; // 格納できたボール

        for j in 0..N {
            // ボールのインデックス
            let k = (i + j) % N;
            let col = C[k];

            // まだ格納されてない
            if boxid[col] == INF {
                // 箱の割当
                boxid[col] = used;
                used += 1;
            }
            if boxid[col] < M && boxes[boxid[col]] < K {
                // 格納
                boxes[boxid[C[k]]] += 1;
                cnt += 1;
            }
        }

        // 出力
        println!("{cnt}");

        // お掃除
        boxid.fill(INF);
        boxes.fill(0);
    }
}
