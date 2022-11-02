//           C - Shopping Street           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc080/tasks/abc080_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1_000_000_000_000_000_000;


// solve
fn main() {
    let N = get!(usize);
    let F = get!(usize;; N);
    let P = get!(isize;; N);

    let mut ans = -1_000_000_000_000_000_000;

    for i in 1..1<<10 {
        let mut tmp = 0;
        for k in 0..N {
            let mut cnt = 0;
            for j in 0..10 {
                // 両方営業しているとき
                if (i>>j)&1 == 1 && F[k][j] == 1 {
                    cnt += 1;
                }
            }
            tmp += P[k][cnt];
        }
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}

