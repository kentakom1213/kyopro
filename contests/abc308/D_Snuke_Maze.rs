//              D - Snuke Maze
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc308/tasks/abc308_d
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
const MOVE: [(usize, usize); 4] = [(1, 0), (0, 1), (NEG1, 0), (0, NEG1)];

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    // BFS
    let mut dist = vec![vec![INF; W]; H];
    dist[0][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((0_usize, 0_usize));

    while let Some((cr, cc)) = q.pop_front() {
        let cur = S[cr][cc];
        for &(dr, dc) in &MOVE {
            let nr = cr.wrapping_add(dr);
            let nc = cc.wrapping_add(dc);
            if nr >= H || nc >= W || dist[nr][nc] < INF {
                continue;
            }
            let nxt = S[nr][nc];
            // 遷移可能性の判定
            if is_next(cur, nxt) {
                q.push_back((nr, nc));
                dist[nr][nc] = dist[cr][cc] + 1;
            }
        }
    }

    debug!(&dist);
    println!("{}", ["Yes", "No"][(dist[H-1][W-1] == INF) as usize]);
}

fn is_next(a: char, b: char) -> bool {
    match (a, b) {
        ('s', 'n') | ('n', 'u') | ('u', 'k') | ('k', 'e') | ('e', 's') => true,
        _ => false,
    }
}
