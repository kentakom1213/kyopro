#![allow(non_snake_case)]

use std::mem::replace;

use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
        UV: [(Usize1, Usize1); N - 1]
    }

    let G = UV.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let mut ans = vec![0; N];
    let mut dp = vec![INF; N];

    rec(0, &mut dp, &mut ans, &A, &G);

    println!("{}", ans.iter().join("\n"));
}

const INF: usize = 1001001001001001001;

fn rec(u: usize, dp: &mut Vec<usize>, ans: &mut [usize], A: &[usize], G: &Vec<Vec<usize>>) {
    // 埋める
    let idx = dp.lower_bound(&A[u]);
    let old = replace(&mut dp[idx], A[u]);
    // LISを求める
    let res = dp.lower_bound(&(INF - 10));
    ans[u] = res;

    for &v in &G[u] {
        if ans[v] != 0 {
            continue;
        }
        rec(v, dp, ans, A, G);
    }
    // 戻す
    dp[idx] = old;
}
