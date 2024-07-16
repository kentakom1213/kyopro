//             Range Sum Query
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
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

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// ## sqrt
    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

/// ## 方針
/// 平方分割を利用
/// - 前処理：O(N)
/// - クエリ：O(√N) * Q
///
/// → 合計で、O(N + Q√N)
fn main() {
    let (N, Q) = get!(usize, usize);

    let B_RATE = 1; // バケットの大きさ / バケットの数
    let B_CNT = (N / B_RATE).sqrt() + 1; // バケットの数
    let B_SIZ = B_RATE * B_CNT; // バケットの大きさ
    let SIZ = B_CNT * B_SIZ; // トータルの大きさ

    // バケットの定義
    let mut buckets = vec![0; B_CNT];
    let mut arr = vec![0; SIZ];

    // クエリの処理 : O(√N)
    for _ in 0..Q {
        let (com, x, y) = get!(usize, usize, usize);
        match com {
            0 => {
                // 更新
                let bucket_idx = x / B_SIZ;
                arr[x] += y;

                let left = bucket_idx * B_SIZ;
                let right = left + B_SIZ;
                buckets[bucket_idx] = 0;
                for i in left .. right {
                    buckets[bucket_idx] += arr[i];
                }
            }
            1 => {
                // 取得
                let y = y + 1;
                let mut sum = 0;

                for k in 0..B_CNT {
                    let (l, r) = (k * B_SIZ, (k + 1) * B_SIZ);
                    if r <= x || y <= l { continue; }
                    if x <= l && r <= y {
                        sum += buckets[k];
                    } else {
                        for i in x.max(l) .. y.min(r) {
                            sum += arr[i];
                        }
                    }
                }
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
