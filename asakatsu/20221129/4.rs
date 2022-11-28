// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d

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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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

// solve
fn main() {
    let N = get!(usize);
    let S = get!(String);

    let mut idx = vec![vec![]; 10];  // 文字`n`は何文字めに存在するか
    for (i, c) in S.chars().enumerate() {
        let n = c.to_digit(10).unwrap() as usize;
        idx[n].push(i);
    }

    // 全ての3桁の番号を調べる
    let mut ans = 0;
    for a in 0..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                // いずれかの文字が存在しない場合はパス
                if idx[a].is_empty() || idx[b].is_empty() || idx[c].is_empty() {
                    continue;
                }

                let fst = idx[a][0];
                let snd_idx = upper_bound(&idx[b], fst);
                if snd_idx == idx[b].len() { continue; }
                let snd = idx[b][snd_idx];

                let trd_idx = upper_bound(&idx[c], snd);
                if trd_idx == idx[c].len() { continue; }
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

/// ソート済み配列において、`v`より大きい最小のインデックスを取得
fn upper_bound(arr: &[usize], v: usize) -> usize {
    let mut ng = -1;
    let mut ok = arr.len() as isize;
    while (ok - ng) > 1 {
        let mid = ((ng + ok) as usize) / 2;
        if v < arr[mid] {
            ok = mid as isize;
        } else {
            ng = mid as isize;
        }
    }
    ok as usize
}
