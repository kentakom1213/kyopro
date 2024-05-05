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

/// # 考察
/// ## パラメータ
/// - 休日の個数
/// - 休日からの距離
/// 
/// ## dp
/// ### 定義
/// ```non-rust-program
/// dp[i][j] := 休日をi個設置し、最後の休日がj日目であるときの生産量の最大値
/// （休日は1日以上設置しないといけないので、0日目には必ず休日がある）
/// ```
/// 
/// ### 更新
/// ```non-rust-program
/// ```
fn main() {
    let N = get!(usize);
    let A = get!(usize;;);

    // Aの累積和配列
    let S = {
        let mut arr = vec![0; N + 1];
        for i in 0..N {
            arr[i+1] = arr[i] + A[i];
        }
        arr
    };

    // 長さ`n`の労働期間が与えられたとき、その労働期間の生産性を求める
    let cost = |n: usize| -> usize {
        let mut ans = S[n/2] * 2;
        if n % 2 == 1 {
            ans += A[(n-1)/2];
        }
        ans
    };

    let mut dp = vec![vec![0; N+1]; N+1];

    // 配列の初期化
    for j in 0..N {
        dp[1][j] = cost(N-1);
    }

    // 更新
    for i in 2..N {
        for j in i-1..N {
            for k in j+1..N {
                chmax!(
                    dp[i][k],
                    dp[i-1][j] + cost(k-j-1) + cost(N-k) - cost(N-j)
                );
            }
        }
    }

    let mut ans = 0;
    for r in &dp {
        chmax!(
            ans,
            *r.iter().max().unwrap()
        );
    }

    println!("{}", ans);
}


