//               E - Souvenir              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc286/tasks/abc286_e
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

// solve
fn main() {
    input! {
        N: usize,
        A: [isize; N],
        S: [Chars; N],
        Q: usize,
        UV: [(Usize1, Usize1); Q],
    }

    // 隣接行列を構築
    // G[i][j] = (最短経路, -得られるお土産の価値の総和の最大値)
    let mut dp = {
        let mut arr = vec![vec![(INF, 0); N]; N];
        for i in 0..N {
            for j in 0..N {
                arr[i][j] = match S[i][j] {
                    'Y' => (1, -A[i]),
                    _ => (INF, -A[i]),
                }
            }
        }
        arr
    };

    // ワーシャル・フロイド法
    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin!(
                    dp[i][j],
                    (
                        dp[i][k].0 + dp[k][j].0,
                        dp[i][k].1 + dp[k][j].1,
                    ),
                );
            }
        }
    }
    
    // クエリ処理
    for &(u, v) in &UV {
        let (d, c) = dp[u][v];
        if d < INF {
            println!("{} {}", d, -c + A[v]);
        } else {
            println!("Impossible");
        }
    }
}
