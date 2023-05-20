//          E - Red and Blue Tree
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc222/tasks/abc222_e
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

/// ## Fp
/// 有限体の実装
pub trait Fp {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Fp for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;

type Edge = (usize, usize); // (行き先, 辺番号)
type Graph = Vec<Vec<Edge>>;

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        K: isize,
        A: [Usize1; M],
        edges: [(Usize1, Usize1); N - 1],
    }

    // グラフの構築
    let mut G: Graph = vec![vec![]; N];
    for (i, &(u, v)) in edges.iter().enumerate() {
        G[u].push((v, i));
        G[v].push((u, i));
    }

    // C[i] := A1→A2, A2→A3, ... の移動を行ったとき，辺iを通る回数
    let mut C = vec![0; N - 1];

    for i in 0..M - 1 {
        let (u, v) = (A[i], A[i + 1]);
        count(u, v, &G, &mut C);
    }

    let S = C.iter().sum::<usize>() as isize;
    
    // (S + K) / 2 が整数にならない
    if S + K < 0 || (S + K) % 2 != 0 {
        println!("0");
        return;
    }

    let R = ((S + K) / 2) as usize;

    // 部分和問題
    // dp[i][j] := C[i]まで見たときに，その和がjとなる組合せの数
    let mut dp = vec![vec![0; R + 1]; N];
    dp[0][0] = 1;

    for i in 0..N - 1 {
        for j in 0..=R {
            if dp[i][j] == 0 {
                continue;
            }
            // C[i]を足さない
            madd!(
                dp[i + 1][j],
                dp[i][j]
            );
            // C[i]を足す
            if j + C[i] <= R {
                madd!(dp[i + 1][j + C[i]], dp[i][j]);
            }
        }
    }

    println!("{}", dp[N - 1][R]);
}

/// 頂点uから頂点vに最短経路で移動するときに辺iを通る回数をC[i]にカウント
fn count(start: usize, end: usize, G: &Graph, C: &mut Vec<usize>) {
    let N = G.len();
    let mut dist = vec![INF; N];
    dist[start] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    let mut prev = vec![(INF, INF); N]; // (直前の頂点, 通った辺)

    // BFS
    while let Some(u) = q.pop_front() {
        for &(v, idx) in &G[u] {
            if dist[v] != INF {
                continue;
            }
            dist[v] = dist[u] + 1;
            prev[v] = (u, idx); // 経路復元用
            q.push_back(v);
        }
    }

    // 経路の復元
    let mut cur = end;
    while cur != start {
        let (p, edge) = prev[cur];
        C[edge] += 1;
        cur = p;
    }
}
