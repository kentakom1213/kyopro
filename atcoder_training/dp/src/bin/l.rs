// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    let mut dp = vec![vec![-INF; N + 1]; N + 1];
    let ans = rec(0, N, &A, &mut dp);

    println!("{}", ans);
}

fn rec(l: usize, r: usize, A: &[isize], memo: &mut Vec<Vec<isize>>) -> isize {
    if memo[l][r] != -INF {
        return memo[l][r];
    }

    if l == r {
        return 0;
    }

    // 自分視点でスコアが最も高くなるものを選択
    let res = (A[l] - rec(l + 1, r, A, memo)).max(A[r - 1] - rec(l, r - 1, A, memo));
    memo[l][r] = res;
    res
}

const INF: isize = 1001001001001001001;
