//           E - Crystal Switches          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc277/tasks/abc277_e
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

// constant
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        UVA: [(Usize1, Usize1, usize); M],
        S: [Usize1; K],
    }

    // グラフの構築
    let G = {
        let mut G = vec![vec![]; N];
        for &(u, v, a) in &UVA {
            G[u].push((v, a == 1));
            G[v].push((u, a == 1));
        }
        G
    };

    // # 状態をもつBFS
    // (次の頂点, スイッチが押されているか)
    let mut que = VecDeque::new();
    que.push_back((0, true));
    // dist[b][i] := (オフ/オン)の状態の頂点iに到達するための最小回数
    let mut dist = vec![vec![INF; N]; 2];
    dist[1][0] = 0;

    while let Some((u, pushed)) = que.pop_front() {
        // dbg!(&dist);
        let state = pushed as usize;
        for &(v, a) in &G[u] {
            if pushed == a && dist[state][v] == INF {
                dist[state][v] = dist[state][u] + 1;
                que.push_back((v, pushed));
            }
        }
        if S.binary_search(&u).is_ok() {
            // スイッチが存在する場合
            for &(v, a) in &G[u] {
                if !pushed == a && dist[state ^ 1][v] == INF {
                    dist[state ^ 1][v] = dist[state][u] + 1;
                    que.push_back((v, !pushed));
                }
            }
        }
    }

    // dbg!(&dist);

    let ans = dist[0][N - 1].min(dist[1][N - 1]);
    if ans < INF {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
