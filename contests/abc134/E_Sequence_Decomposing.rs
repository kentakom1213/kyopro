//         E - Sequence Decomposing        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc134/tasks/abc134_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize; N);

    // LIS配列
    let mut dp = vec![INF; N];

    // 逆順にLIS（広義単調減少列）を求める
    for &v in A.iter().rev() {
        let idx = upper_bound(&dp, v);
        dp[idx] = v;
    }

    // LISの長さを測る
    let ans = dp.iter().take_while(|&&v| v < INF).count();
    println!("{}", ans);
}

/// ソート済み配列において、`v`以上の最小のインデックスを取得
fn lower_bound(arr: &[usize], v: usize) -> usize {
    let mut ng = -1;
    let mut ok = arr.len() as isize;
    while (ok - ng) > 1 {
        let mid = ((ng + ok) as usize) / 2;
        if v <= arr[mid] {
            ok = mid as isize;
        } else {
            ng = mid as isize;
        }
    }
    ok as usize
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
