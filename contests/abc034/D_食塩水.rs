//                 D - 食塩水                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc034/tasks/abc034_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque};

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
 * # 方針
 * - 平均の最大化 → 2分探索
 * 
 * ## 参考
 * - https://tutuz.hateblo.jp/entry/2018/07/20/232347
 */

// solve
fn main() {
    let (N, K) = get!(usize, usize);
    let wp = get!(f64, f64; N);

    let isOK = |m: f64| -> bool {
        let mut calc = vec![0.0; N];
        for i in 0..N {
            let (w, p) = wp[i];
            calc[i] = w * (p - m);
        }
        calc.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut res = 0.0;
        for i in N-K..N {
            res += calc[i];
        }
        res >= 0.0
    };

    // 2分探索
    const EPS: f64 = 0.000_000_000_1;
    let mut ok = 0.0;
    let mut ng = 100.0;
    while ng - ok > EPS {
        let mid = (ok + ng) / 2.0;
        if isOK(mid) {
            ok = mid;
        } else {
            ng = mid
        }
    }

    println!("{}", ok);
}

