//           F - Dungeon Explore
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc305/tasks/abc305_f
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

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

// main
fn main() {
    let (N, M) = get!(usize, usize);

    let mut visited = vec![false; N];
    visited[0] = true;
    let mut stack = vec![0];

    // DFS
    'out: while let Some(nxt) = get_next() {
        for &v in &nxt {
            if visited[v] {
                continue;
            }
            // vに移動
            stack.push(v);
            visited[v] = true;
            println!("{}", v + 1);
            continue 'out;
        }
        stack.pop();
        let par = stack.pop().unwrap();
        println!("{}", par + 1);
        stack.push(par);
    }
}

fn get_next() -> Option<Vec<usize>> {
    let S = get!(String);
    if &S == "OK" || &S == "-1" {
        return None;
    }
    let v = S
        .split_whitespace()
        .skip(1)  // 個数の情報を無視
        .map(|v| v.parse::<usize>().unwrap())
        .map(|v| v - 1)  // 0-indexedに
        .collect_vec();
    Some(v)
}
