//             D - Moving Piece
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc175/tasks/abc175_d
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
const IINF: isize = 1001001001001001001;
const UINF: usize = 1001001001001001001;

/// ## 方針
/// - 各頂点からDFS
/// - ループ検出
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [Usize1; N],
        C: [isize; N],
    }

    let mut ans = -IINF;

    for i in 0..N {
        ans = ans.max(get_max(i, K, &P, &C));
    }

    println!("{}", ans);
}

/// startから始めたときにえられるコストの最大値
fn get_max(start: usize, K: usize, P: &[usize], C: &[isize]) -> isize {
    let N = P.len();
    let mut idx = vec![UINF; N];
    let mut scores = vec![-IINF; N];

    // ループ検出
    let mut i = 0;
    let mut u = start;
    let mut acc = 0;
    let mut has_loop = false;

    // 1周目
    while i < K {
        if scores[u] != -IINF {
            has_loop = true;
            break;
        }
        // インデックスの更新
        idx[u] = i;
        i += 1;
        // 累積の更新
        acc += C[u];
        scores[u] = acc;
        // ポインタの更新
        u = P[u];
    }

    if has_loop {
        // ループの情報
        let loop_begin = u; // ループの始まり
        let loop_size = i - idx[loop_begin]; // ループの長さ
        let loop_total = (0..loop_size)
            .fold((0, loop_begin), |(acc, u), _| (acc + C[u], P[u]))
            .0; // 1周あたりの合計

        debug!(loop_begin, loop_size, loop_total);

        // 2周目
        let mut u = loop_begin;
        for _ in 0..loop_size {
            // 何周余分にできるか
            let rem = (K - idx[u] - 1) / loop_size;

            let with_loop = rem as isize * loop_total;

            scores[u] = scores[u].max(scores[u] + with_loop);

            u = P[u];
        }
    }

    debug!(start, &scores);

    *scores.iter().max().unwrap()
}
