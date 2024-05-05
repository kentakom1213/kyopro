//          E - Count Simple Paths         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc284/tasks/abc284_e
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
const LIMIT: usize = 1_000_000;

// solve
fn main() {
    input! {
        (N, M): (usize, usize),
        edges: [(Usize1, Usize1); M],
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // dfsで数え上げ
    let mut ans = 1;
    let mut is_visited = vec![false; N];
    let mut path = vec![];

    dfs(0, &mut ans, &mut is_visited, &mut path, &G);

    println!("{}", ans.min(LIMIT));
}

fn dfs(cur: usize, ans: &mut usize, is_visited: &mut Vec<bool>, path: &mut Vec<usize>, G: &Vec<Vec<usize>>) {
    if *ans > LIMIT { return; }

    // println!("{:?}", path);
    is_visited[cur] = true;
    path.push(cur);

    for &nxt in &G[cur] {
        if !is_visited[nxt] {
            *ans += 1;
            dfs(nxt, ans, is_visited, path, G);
        }
    }

    is_visited[cur] = false;
    path.pop();
}
