//            B - AtCoderでじゃんけんを           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc048/tasks/arc048_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

const WIN: usize = 0;
const LOSE: usize = 1;
const DRAW: usize = 2;

const G: usize = 0;
const C: usize = 1;
const P: usize = 2;

// solve
fn main() {
    let N = get!(usize);
    let RHi: Vec<(usize, usize, usize)> = {
        let mut arr = get!(usize, usize; N)
            .iter()
            .enumerate()
            .map(|(i, &(r, h))| (r, h-1, i))
            .collect::<Vec<(usize, usize, usize)>>();
        // ソート
        arr.sort();
        // 番兵
        arr.push((INF, 0, 0));

        arr
    };

    // 解を保存
    // res[i] := [勝ち, 負け, 引き分け] の数
    let mut res = vec![vec![0; 3]; N];
    
    let mut cur_rate = 0;
    let mut cnt_prev = 0;
    let mut same_rate: Vec<(usize, usize)> = vec![];
    let mut hand = [0, 0, 0];  // [グー, チョキ, パー]
    for (cur, &(r, h, i)) in RHi.iter().enumerate() {
        if cur_rate < r {
            // 同じレートの人を処理（償却線形時間）
            let cnt_next = N - cur;

            for &(j, h2) in &same_rate {
                // 自分よりレートが高い人
                res[j][LOSE] += cnt_next;

                // じゃんけんの結果を処理
                res[j][WIN] += hand[(h2 + 1) % 3];
                res[j][LOSE] += hand[(h2 + 2) % 3];
                res[j][DRAW] += hand[h2] - 1;
            }
            // 現在のレートを更新
            cnt_prev = cur;
            cur_rate = r;
            same_rate.clear();
            hand = [0, 0, 0];
        }
        if r == INF { break; }

        hand[h] += 1;
        same_rate.push((i, h));

        // レートが違う場合を処理
        res[i][WIN] += cnt_prev;  // 自分よりレートが下には勝つ
    }

    for row in &res {
        println!("{} {} {}", row[0], row[1], row[2]);
    }
}
