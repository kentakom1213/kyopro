//            D - Peaceful Teams
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc310/tasks/abc310_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        T: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }

    let members = (0..N).collect_vec();

    let mut ans = 0;

    for perm in dfs(N - 1, N, T) {
        debug!(&perm, is_ok(&perm, N, T, &AB));

        if is_ok(&perm, N, T, &AB) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

/// すべてのグループ分けを生成する
fn dfs(i: usize, N: usize, T: usize) -> impl Iterator<Item = Vec<Vec<usize>>> {
    if i == 0 {
        return vec![vec![vec![0]]].into_iter();
    }
    // 一つ前までの順列をすべて生成
    let mut res = vec![];

    for rest_perm in dfs(i - 1, N, T) {
        // すでにあるグループに追加する
        for g in 0..rest_perm.len() {
            let mut tmp = rest_perm.clone();
            tmp[g].push(i);
            res.push(tmp);
        }

        // 新しいグループを作る
        if rest_perm.len() < T {
            let mut tmp = rest_perm.clone();
            tmp.push(vec![i]);
            res.push(tmp);
        }
    }
    res.into_iter()
}

/// グループ分けが条件を満たすかを判定する
fn is_ok(group: &Vec<Vec<usize>>, N: usize, T: usize, AB: &Vec<(usize, usize)>) -> bool {
    if group.len() < T {
        return false;
    }

    // グループの彩色
    let mut col = vec![INF; N];

    for (i, g) in group.iter().enumerate() {
        for &m in g {
            col[m] = i;
        }
    }

    AB.iter().all(|&(a, b)| col[a] != col[b])
}
