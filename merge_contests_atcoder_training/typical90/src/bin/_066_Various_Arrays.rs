//         066 - Various Arrays（★5）        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_bn
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

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

/* 

## 方針
- 期待値の線形性
- 全ての(i,j)の組について、(ai,aj)が反転する期待値を求める

## 反転する組合せ


 */

// solve
fn main() {
    let N = get!(usize);
    let LR = get!(isize, isize; N);

    let mut ans = 0.0;
    for i in 0..N {
        for j in i+1..N {
            // a[i] > a[j] となる期待値
            let (Li, Ri) = LR[i];
            let (Lj, Rj) = LR[j];
            let comb = ((Ri - Li + 1) * (Rj - Lj + 1)) as f64;
            let mut cnt = 0.0;
            for ai in Li..=Ri {
                for aj in Lj..=Rj {
                    if ai > aj {
                        cnt += 1.0;
                    }
                }
            }
            ans += cnt / comb;
        }
    }

    println!("{}", ans);
}

