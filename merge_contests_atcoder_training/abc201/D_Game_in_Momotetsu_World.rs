//       D - Game in Momotetsu World       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc201/tasks/abc201_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr, $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr, $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

// solve
fn main() {
    let (H, W) = get!(usize, usize);
    let A: Vec<Vec<isize>> = (0..H).map(
        |_| get!(String)
            .chars()
            .map(|c| if c == '+' { 1 } else { -1 })
            .collect()
    ).collect();

    // dp配列
    // opt[i][j] := 両者が最適に行動したときの (高橋の得点)-(青木の得点)
    let mut opt = vec![vec![0_isize; W]; H];

    // 更新
    for i in (0..H).rev() {
        for j in (0..W).rev() {
            // どちらのターンかを判定
            let is_takahashi = (i + j) % 2 == 0;

            if i+1 == H && j+1 == W {
                opt[i][j] = 0;
                continue;
            }

            if is_takahashi {
                opt[i][j] = -INF;

                // opt[i][j]を最大化する
                // opt[i][j] = max{ opt[i][j+1] + A[i][j+1], opt[i+1][j] + A[i+1][j] }
                if j + 1 < W {
                    chmax!(
                        opt[i][j],
                        opt[i][j+1] + A[i][j+1],
                    );
                }
                if i + 1 < H {
                    chmax!(
                        opt[i][j],
                        opt[i+1][j] + A[i+1][j],
                    );
                }
            } else {
                opt[i][j] = INF;

                // opt[i][j]を最小化する
                // opt[i][j] = min{ opt[i][j+1] - A[i][j+1], opt[i+1][j] - A[i+1][j] }
                if j + 1 < W {
                    chmin!(
                        opt[i][j],
                        opt[i][j+1] - A[i][j+1],
                    );
                }
                if i + 1 < H {
                    chmin!(
                        opt[i][j],
                        opt[i+1][j] - A[i+1][j],
                    );
                }
            }
        }
    }

    if opt[0][0] == 0 {
        println!("Draw");
    } else if opt[0][0] > 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
