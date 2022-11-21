//                B - 赤と黒の木                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc024/tasks/arc024_2
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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;


// solve
fn main() {
    let N = get!(usize);
    let colors = get!(usize; N);  // 0:黒, 1:赤
    let sum = colors.iter().fold(0, |acc, v| acc + v);

    // 全てが同じ色のとき
    if sum == 0 || sum == N {
        println!("-1");
        return;
    }

    // その他の場合は収束する
    // 最大の長さの同じ色の列を見つける
    // 円環にするため、2個連結する
    let mut cur = colors[0];
    let mut cnt = 0;
    let mut ans = 0;
    for &v in colors.iter().chain(colors.iter()) {
        if cur == v {
            cnt += 1;
        } else {
            cur = v;
            ans = ans.max(cnt);
            cnt = 1;
        }
    }
    ans = ans.max(cnt);

    println!("{}", (ans + 1)/2);
}
