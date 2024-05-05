//             E - Work or Rest            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc285/tasks/abc285_e
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
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
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
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    // B[i] := 平日がi日連続しているときの生産性
    let B= {
        let mut arr = vec![0; N];
        for i in 1..N {
            arr[i] = arr[i-1] + A[(i-1)/2];
        }
        arr
    };

    // dp[i][j] := 現在の曜日i、現在連続している平日jのときの生産性の最大値
    let mut dp = vec![vec![None; N]; N+1];
    dp[1][0] = Some(B[0]);  // = 0

    // 遷移（配るDP）
    for i in 0..N {
        for j in 0..N-1 {
            // 休日を設定しない
            if let Some(v) = dp[i][j] {
                chmax!(
                    dp[i+1][j+1],
                    Some(v)
                );
            }

            // j日目を休日を設定する
            if let Some(v) = dp[i][j] {
                chmax!(
                    dp[i+1][0],
                    Some(v + B[j])
                );
            }
        }
    }

    let ans = dp[N].iter()
        .enumerate()
        .filter(|(i, &v)| v != None)
        .map(|(i, &v)| {
            v.unwrap() + B[i]
        })
        .max()
        .unwrap();
    
    println!("{}", ans);
}
