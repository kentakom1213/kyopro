//                C - 幅優先探索                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc007/tasks/abc007_3
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
const MOVE: [(usize, usize); 4] = [(1, 0), (0, 1), (NEG1, 0), (0, NEG1)];

// main
fn main() {
    input! {
        R: usize,
        C: usize,
        sr: Usize1,
        sc: Usize1,
        gr: Usize1,
        gc: Usize1,
        S: [Chars; R],
    }

    let mut q = VecDeque::new();
    q.push_back((sr, sc));
    let mut dist = vec![vec![INF; C]; R];
    dist[sr][sc] = 0;

    while let Some((r, c)) = q.pop_front() {
        for &(dr, dc) in &MOVE {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if S[nr][nc] == '#' || dist[nr][nc] < INF {
                continue;
            }
            q.push_back((nr, nc));
            dist[nr][nc] = dist[r][c] + 1;
        }
    }

    println!("{}", dist[gr][gc]);
}
