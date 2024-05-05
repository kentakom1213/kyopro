//        C - Successive Subtraction       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

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
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    // 正/負に分ける
    let (mut pos, mut neg) = A
        .iter()
        .fold((vec![], vec![]), |mut acc, &v| {
            if v >= 0 {
                acc.0.push(v);
            } else {
                acc.1.push(v);
            };
            acc
        });
    
    // 絶対値の降順でソート
    pos.sort_by_key(|&v| Reverse(v));
    neg.sort();
    debug!(&pos, &neg);

    // 操作手順
    let mut ans = vec![];

    // 正の要素、負の要素だけの場合
    if pos.is_empty() {
        let x = neg.pop().unwrap();
        let y = neg.pop().unwrap();
        ans.push((y, x));
        pos.push(y - x);
    }
    else if neg.is_empty() {
        let x = pos.pop().unwrap();
        let y = pos.pop().unwrap();
        if pos.len() == 0 {
            println!("{}", y - x);
            println!("{} {}", y, x);
            return;
        }
        ans.push((y, x));
        neg.push(y - x);
    }

    debug!(&pos, &neg);

    // 正の要素を一つだけ残して負の要素にマージ
    while pos.len() > 1 && !neg.is_empty() {
        let p = pos.pop().unwrap();
        let n = neg.pop().unwrap();
        ans.push((n, p));
        neg.push(n - p);
    }

    // 負の要素を正の要素にマージ
    while let Some(n) = neg.pop() {
        let p = pos[0];
        ans.push((p, n));
        pos[0] -= n;
    }

    debug!(&pos, &neg);

    println!("{}", pos[0]);
    println!("{}", ans.iter().map(|&(x, y)| format!("{} {}", x, y)).join("\n"));
}
