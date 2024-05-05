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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}
/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [
    (0, 1),
    (1, 0),
    (NEG1, 0),
    (0, NEG1),
];

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        T: usize,
        A: [Chars; H],
    }

    // スタートとゴールを探索
    let mut S = (0, 0);
    let mut G = (0, 0);
    let mut points = vec![];
    for i in 0..H {
        for j in 0..W {
            match A[i][j] {
                'S' => S = (i, j),
                'G' => G = (i, j),
                'o' => points.push((i, j)),
                _ => (),
            }
        }
    }

    // 主要な頂点同士の最短経路を探索する
    let P = points.len();
    let mut dists = vec![vec![INF; P]; P];
    for (i, &p) in points.iter().enumerate() {
        let d = dist(p, &A);
        for (j, &q) in points.iter().enumerate() {
            dists[i][j] = d[q.0][q.1];
            dists[j][i] = dists[i][j];
        }
    }

    let dist_S = dist(S, &A);
    let dist_G = dist(G, &A);

    if dist_S[G.0][G.1] > T {
        println!("-1");
        return;
    }

    // bitDP
    // dp[i][S] := 今までに訪れた頂点の集合がS、最後に訪れた頂点がiのとき、距離の最小値
    let mut dp = vec![vec![INF; P]; 1 << P];

    // スタートをSに固定
    for (i, &p) in points.iter().enumerate() {
        dp[1 << i][i] = dist_S[p.0][p.1];
    }

    for s in 0..1 << P {
        for u in 0..P {
            for v in 0..P {
                // vをまだ訪れていないとき
                if s & (1 << v) == 0 {
                    chmin!(
                        dp[s | 1 << v][v],
                        dp[s][u] + dists[u][v],
                    );
                }
            }
        }
    }

    // bit全探索
    let mut ans = 0;

    // S -> o -> ... -> G
    // を満たすルートの中で、距離の合計がT以下のもの
    for i in 0..1 << P {
        let mut route_min = INF;  // 最小のルート
        for (j, &p) in points.iter().enumerate() {
            // jで終わるもの
            chmin!(
                route_min,
                dp[i][j] + dist_G[p.0][p.1],
            );
        }
        if route_min <= T {
            chmax!(
                ans,
                i.count_ones(),  // 訪れたお菓子の数
            );
        }
    }

    println!("{}", ans);
}

/// ます(i,j)からスタートしたときの全てのマスへの最短経路
fn dist(S: (usize, usize), A: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let (H, W) = (A.len(), A[0].len());
    let mut res = vec![vec![INF; W]; H];
    res[S.0][S.1] = 0;
    let mut que = VecDeque::new();
    que.push_back(S);

    while let Some((r, c)) = que.pop_front() {
        for &(dr, dc) in &MOVE {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr >= H || nc >= W { continue; }
            if A[nr][nc] == '#' { continue; }
            if res[nr][nc] == INF {
                res[nr][nc] = res[r][c] + 1;
                que.push_back((nr, nc));
            }
        }
    }

    res
}
