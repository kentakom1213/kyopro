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
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const ADJ4: [(usize, usize); 4] = [(NEG1, NEG1), (NEG1, 1), (1, NEG1), (1, 1)];

fn main() {
    input! {
        N: usize,
        ar: Usize1,
        ac: Usize1,
        br: Usize1,
        bc: Usize1,
        S: [Chars; N]
    }

    // dist[d][i][j] := 方向dを向いている状態でマスi,jにいるときの距離
    let mut dist = vec![vec![vec![INF; N]; N]; 4];

    // 0-1BFS
    let mut q = VecDeque::new();

    for d in 0..4 {
        dist[d][ar][ac] = 1;
        q.push_back(((ar, ac), d));
    }

    while let Some(((r, c), d)) = q.pop_front() {
        // その方向に1すすむ（距離0）
        let (dr, dc) = ADJ4[d];
        let nr = r.wrapping_add(dr);
        let nc = c.wrapping_add(dc);
        if nr < N && nc < N && S[nr][nc] == '.' && dist[d][nr][nc] > dist[d][r][c] {
            dist[d][nr][nc] = dist[d][r][c];
            q.push_front(((nr, nc), d));
        }
        // 方向転換（距離1）
        for nd in 0..4 {
            if d != nd && dist[nd][r][c] == INF {
                dist[nd][r][c] = dist[d][r][c] + 1;
                q.push_back(((r, c), nd));
            }
        }
    }

    if cfg!(debug_assertions) {
        for d in 0..4 {
            for row in &dist[d] {
                eprintln!(
                    "{}",
                    row.iter()
                        .map(|&x| if x == INF {
                            "∞".to_string()
                        } else {
                            x.to_string()
                        })
                        .join(" ")
                );
            }
            eprintln!();
        }
    }

    // 答え
    let ans = (0..4).map(|d| dist[d][br][bc]).min().unwrap();

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
