//           D - joisino's travel          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc073/tasks/abc073_d
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
use itertools::{self, Itertools};

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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - N <= 200
/// - R <= 8
/// ---
/// - 全頂点対の最短経路を求めておく
/// - 順列の全探索
fn main() {
    input! {
        (N, M, R): (usize, usize, usize),
        Rs: [Usize1; R],
        edges: [(Usize1, Usize1, usize); M],
    }

    // ワーシャル・フロイド法
    let mut dist = vec![vec![INF; N]; N];
    for &(u, v, c) in &edges {
        dist[u][v] = c;
        dist[v][u] = c;
    }
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin! {
                    dist[i][j],
                    dist[i][k] + dist[k][j],
                };
            }
        }
    }

    // 順列全探索
    let mut ans = INF;
    for perm in Rs.iter().permutations(R) {
        let mut tmp = 0;
        for i in 1..R {
            let (&u, &v) = (perm[i-1], perm[i]);
            tmp += dist[u][v];
        }
        chmin!(ans, tmp);
    }

    println!("{}", ans);
}
