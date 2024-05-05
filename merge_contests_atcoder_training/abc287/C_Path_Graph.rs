//             C - Path Graph?             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc287/tasks/abc287_c
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
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
        G[v].push(u);
    }

    // 木であるか
    let is_tree = N - 1 == M;

    // 連結性の判定
    let mut visited = vec![false; N];
    visited[0] = true;
    let mut st = vec![0];
    while let Some(u) = st.pop() {
        for &v in &G[u] {
            if visited[v] {continue;}
            visited[v] = true;
            st.push(v);
        }
    }

    let is_connected = visited.iter().all(|v| *v);

    // 次数が2以下であるか
    let is_dem_le_2 = G
        .iter()
        .all(|edges| edges.len() <= 2);
    
    if is_tree && is_connected && is_dem_le_2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
