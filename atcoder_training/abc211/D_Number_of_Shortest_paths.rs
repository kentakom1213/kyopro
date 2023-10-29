//       D - Number of Shortest paths      
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc211/tasks/abc211_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
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

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }

    let mut G = vec![vec![]; N];
    for &(a, b) in &AB {
        G[a].push(b);
        G[b].push(a);
    }

    // BFSで最短経路を探索
    let mut que = VecDeque::new();
    que.push_back(0);
    let mut dist = vec![INF; N];
    dist[0] = 0;

    // dp
    let mut dp = vec![0; N];
    dp[0] = 1;

    while let Some(u) = que.pop_front() {
        for &v in &G[u] {
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                que.push_back(v);
                dp[v] = dp[u];
            } else if dist[v] == dist[u] + 1 {
                dp[v] = dp[v].madd(dp[u]);
            }
        }
    }

    println!("{}", dp[N-1]);
}
