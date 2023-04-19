//           E - Magical Ornament          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc190/tasks/abc190_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use std::collections::VecDeque;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

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

/// ## 方針
/// - K <= 17
/// - bitDPをする
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
        K: usize,
        C: [Usize1; K],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(a, b) in &AB {
        G[a].push(b);
        G[b].push(a);
    }

    // Cの要素同士の距離をBFSで全探索しておく
    let dist = {
        let mut dist = vec![vec![INF; N]; K];
        for i in 0..K {
            let mut que = VecDeque::new();
            que.push_back(C[i]);
            dist[i][C[i]] = 0;
            while let Some(cur) = que.pop_front() {
                for &nxt in &G[cur] {
                    if dist[i][nxt] == INF {
                        dist[i][nxt] = dist[i][cur] + 1;
                        que.push_back(nxt);
                    }
                }
            }
        }
        dist
    };

    // bitDP
    // dp[S][v]: 集合Sに含まれる頂点をすべて通り、最後に頂点vにいるときの最小コスト
    let mut dp = vec![vec![INF; K]; 1 << K];
    for i in 0..K {
        dp[1 << i][i] = 1;
    }

    // 更新処理
    for S in 0..1 << K {
        for u in 0..K {
            for v in 0..K {
                // まだ通過していないとき
                if S >> v & 1 == 0 {
                    chmin! {
                        dp[S | 1 << v][v],
                        dp[S][u] + dist[u][C[v]]
                    };
                }
            }
        }
    }

    let ans = *dp[(1 << K) - 1].iter().min().unwrap();

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}