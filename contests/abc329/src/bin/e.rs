// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

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

fn main() {
    input! {
        N: usize,
        M: usize,
        mut S: String,
        T: String
    }

    let fill = "#".repeat(M);

    // '#'をワイルドカードとみなして貪欲に書き換えていく

    // 書き換えが行われる可能性がある位置
    let mut que = VecDeque::new();

    for i in 0..N - M + 1 {
        if &S[i .. i + M] == &T {
            que.push_back(i);
        }
    }

    let mut rem = N;
    let mut is_visited = vec![false; N];

    while let Some(begin) = que.pop_front() {
        is_visited[begin] = true;
        if is_match(begin, &S, &T) {
            rem -= &S[begin..begin + M].chars().filter(|&c| c != '#').count();
            S.replace_range(begin..begin + M, &fill);
            // 区間に追加
            for i in begin.saturating_sub(M)..=(N - 1).min(begin + M - 1) {
                if i + M <= N && !is_visited[i] && is_match(i, &S, &T) {
                    que.push_back(i);
                }
            }
        }
        if rem == 0 {
            break;
        }

        debug!(S, que);
    }

    if S.chars().map(|c| c == '#').all(|x| x) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_match(begin: usize, S: &str, T: &str) -> bool {
    T.chars()
        .zip(S[begin..].chars())
        .all(|(t, s)| s == '#' || s == t)
}
