//      D - Face Produces Unhappiness
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc140/tasks/abc140_d
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

/// ## RunLengthEncode
/// ランレングス圧縮 (from Iterator)
fn run_length_encode_from<T, I>(mut itr: I) -> Vec<(T, usize)>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    let mut res = vec![];
    let mut cur = itr.next().unwrap();
    let mut cnt = 1;
    for val in itr {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cur, cnt));
            cur = val;
            cnt = 1;
        }
    }
    res.push((cur, cnt));

    res
}

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        S: String,
    }

    let mut q = run_length_encode_from(S.chars())
        .into_iter()
        .map(|(_, l)| l)
        .collect_vec();
    debug!(&q);

    // 区間をマージする操作
    for _ in 0..K {
        let a = q.pop().unwrap_or(0);
        let b = q.pop().unwrap_or(0);
        let c = q.pop().unwrap_or(0);
        q.push(a + b + c);
    }

    // 要素から1引いたものの和を求める
    let ans = q.iter().map(|&v| v - 1).sum::<usize>();

    println!("{}", ans);
}
