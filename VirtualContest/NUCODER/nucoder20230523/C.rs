// https://atcoder.jp/contests/abc235/tasks/abc235_d

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

// main
fn main() {
    input! {
        a: usize,
        N: usize,
    }

    // BFS
    let mut q = VecDeque::new();
    q.push_front(N);
    let mut dist = vec![INF; 10_000_000];
    dist[N] = 0;

    while let Some(u) = q.pop_front() {
        if u % a == 0 {
            let v = u / a;
            if dist[v] == INF {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
        {
            let s = u.to_string();
            if s.len() < 2 || &s[1..2] == "0" {
                continue;
            }
            let v = s[1..].to_string() + &s[..1];
            let v = v.parse::<usize>().unwrap();
            if dist[v] != INF {
                continue;
            }
            debug!(u, v);
            dist[v] = dist[u] + 1;
            q.push_back(v);
        }
    }

    debug!(&dist[..100]);

    if dist[1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[1]);
    }
}
