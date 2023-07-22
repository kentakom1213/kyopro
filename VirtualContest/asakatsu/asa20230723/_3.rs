// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use petgraph::visit::VisitMap;
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
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    // BFS
    let mut visited = vec![vec![false; W]; H];
    visited[0][0] = true;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((0, 0));

    while let Some((r, c)) = q.pop_front() {
        for &(dr, dc) in &MOVE {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < H && nc < W && !visited[nr][nc] && (
                S[r][c] == 's' && S[nr][nc] == 'n'
                || S[r][c] == 's' && S[nr][nc] == 'n'
                || S[r][c] == 'n' && S[nr][nc] == 'u'
                || S[r][c] == 'u' && S[nr][nc] == 'k'
                || S[r][c] == 'k' && S[nr][nc] == 'e'
                || S[r][c] == 'e' && S[nr][nc] == 's'
            ) {
                visited[nr][nc] = true;
                q.push_back((nr, nc));
            }
        }
    }

    println!("{}", ["No", "Yes"][visited[H - 1][W - 1] as usize]);
}
