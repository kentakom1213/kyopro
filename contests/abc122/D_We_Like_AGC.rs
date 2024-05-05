//             D - We Like AGC             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc122/tasks/abc122_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

const CHARS: [char; 4] = ['A', 'C', 'G', 'T'];

// main
fn main() {
    input! {
        N: usize,
    }

    let mut memo = BTreeMap::new();
    let ans = dfs(N, ['_', '_', '_'], &mut memo);

    println!("{}", ans);
}

/// 残りn文字で、直前の文字列がprevであるような
fn dfs(n: usize, prev: [char; 3], memo: &mut BTreeMap<(usize, [char; 3]), usize>) -> usize {
    if let Some(&ans) = memo.get(&(n, prev)) {
        return ans;
    }
    if n == 0 {
        return 1;
    }
    // 枝刈りパート
    let mut ans = match prev {
        [_, 'A', 'G'] => {
            // 'C'以外があとにくる
            dfs(n - 1, ['A', 'G', 'A'], memo)
            + dfs(n - 1, ['A', 'G', 'G'], memo)
            + dfs(n - 1, ['A', 'G', 'T'], memo)
        },
        [_, 'G', 'A'] => {
            // 'C'以外があとにくる
            dfs(n - 1, ['G', 'A', 'A'], memo)
            + dfs(n - 1, ['G', 'A', 'G'], memo)
            + dfs(n - 1, ['G', 'A', 'T'], memo)
        },
        [_, 'A', 'C'] => {
            // 'G'以外があとにくる
            dfs(n - 1, ['A', 'C', 'A'], memo)
            + dfs(n - 1, ['A', 'C', 'C'], memo)
            + dfs(n - 1, ['A', 'C', 'T'], memo)
        },
        ['A', x @ _, 'G'] => {
            // 'C'以外があとにくる
            dfs(n - 1, [x, 'G', 'A'], memo)
            + dfs(n - 1, [x, 'G', 'G'], memo)
            + dfs(n - 1, [x, 'G', 'T'], memo)
        },
        ['A', 'G', x @ _] => {
            // 'C'以外があとにくる
            dfs(n - 1, ['G', x, 'A'], memo)
            + dfs(n - 1, ['G', x, 'G'], memo)
            + dfs(n - 1, ['G', x, 'T'], memo)
        },
        [_, y, z] => {
            dfs(n - 1, [y, z, 'A'], memo)
            + dfs(n - 1, [y, z, 'C'], memo)
            + dfs(n - 1, [y, z, 'G'], memo)
            + dfs(n - 1, [y, z, 'T'], memo)
        },
    };
    ans %= MOD1;
    memo.insert((n, prev), ans);
    ans
}
