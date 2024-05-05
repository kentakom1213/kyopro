//             E - Transitivity            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc292/tasks/abc292_e
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

/// ## 解法
/// - 有向グラフを、各頂点を根とする木の森として考える
/// - 森の辺の数 - 現在の辺の数 = 答え
fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &UV {
        G[u].push(v);
    }

    // それぞれの頂点に関してDFS → 到達できる頂点について考える
    let mut ans = 0;

    for i in 0..N {
        // DFS
        let mut st = vec![i];
        let mut is_visited = vec![false; N];
        is_visited[i] = true;

        while let Some(u) = st.pop() {
            for &v in &G[u] {
                if is_visited[v] {continue;}
                st.push(v);
                is_visited[v] = true;
                ans += 1;
            }
        };
    }

    ans -= M;

    println!("{}", ans);
}
