//           D - Snuke Panic (1D)          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc266/tasks/abc266_d
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
static MAX: usize = 101010;

// solve
fn main() {
    let N = get!(usize);
    let mut sunuke = HashMap::new();
    for i in 0..N {
        let (t, x, a) = get!(usize, usize, usize);
        sunuke.insert((t, x), a);
    }

    // dp[i][j] := 時刻iの時点で座標jにいるとき、得られるsunukeの最大値
    let mut dp = vec![vec![None; 5]; MAX];
    dp[0][0] = Some(0);

    for t in 1..MAX {
        for x in 0..5 {

            // そのまま
            if dp[t-1][x] != None {
                let new = Some(
                        dp[t-1][x].unwrap()
                        + sunuke.get(&(t, x)).unwrap_or(&0)
                );
                if dp[t][x] < new {
                    dp[t][x] = new;
                }
            }

            // 1つ前から移動
            if x > 0 && dp[t-1][x-1] != None {
                let new = Some(
                        dp[t-1][x-1].unwrap()
                        + sunuke.get(&(t, x)).unwrap_or(&0)
                );
                if dp[t][x] < new {
                    dp[t][x] = new;
                }
            }

            // 1つ後から移動
            if x < 4 && dp[t-1][x+1] != None {
                let new = Some(
                        dp[t-1][x+1].unwrap()
                        + sunuke.get(&(t, x)).unwrap_or(&0)
                );
                if dp[t][x] < new {
                    dp[t][x] = new;
                }
            }
        }
    }

    println!("{}",
        dp[MAX-1]
            .iter()
            .map(|v| v.unwrap_or(0))
            .max()
            .unwrap()
    );
}
