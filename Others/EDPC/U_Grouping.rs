//               U - Grouping
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_u
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
const INF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        A: [[isize; N]; N],
    }

    let mut memo = vec![None; 1 << N];
    let ans = f((1 << N) - 1, N, &A, &mut memo);

    println!("{}", ans);
}

// 集合Sのうさぎをグループに分割するときの総得点の最大値
fn f(S: usize, N: usize, A: &Vec<Vec<isize>>, memo: &mut Vec<Option<isize>>) -> isize {
    if let Some(val) = memo[S] {
        return val;
    }

    if S.count_ones() == 1 {
        memo[S] = Some(0);
        return 0;
    }

    let mut ans = -INF;

    // 分割しないとき
    let mut tmp = 0;
    for i in 0..N {
        for j in i + 1..N {
            if (S >> i) & 1 == 1 && (S >> j) & 1 == 1 {
                tmp += A[i][j];
            }
        }
    }
    ans = ans.max(tmp);

    // 分割するとき
    let mut T = S - 1;
    loop {
        T &= S;
        if T == 0 {
            break;
        }
        let tmp = f(T, N, A, memo) + f(S ^ T, N, A, memo);
        ans = ans.max(tmp);
        T -= 1;
    }

    memo[S] = Some(ans);
    ans
}
