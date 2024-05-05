//             E - At Least One            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc260/tasks/abc260_e
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
    ($t:ty) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
    }};
    ($($t:ty),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
    }};
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
    ($t:ty ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
    }};
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
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 尺取り法
fn main() {
    let (N, M) = get!(usize, usize);
    let ab = get!(usize, usize; N);

    let mut inv = vec![vec![]; M+1];
    for (i, &(a, b)) in ab.iter().enumerate() {
        inv[a].push(i);
        inv[b].push(i);
    }

    // 尺取り法
    let mut r = 1;
    let mut cnt_zero = N;  // 条件を満たさないiの個数
    let mut cnt = vec![0; N];  // 区間に含まれるiの個数
    let mut ans = vec![0; M+3];

    for l in 1..=M {
        while r <= M && cnt_zero != 0 {
            // 区間に追加する処理
            for &x in &inv[r] {
                if cnt[x] == 0 {
                    cnt_zero -= 1;
                }
                cnt[x] += 1;
            }
            r += 1;
        }

        if cnt_zero != 0 { break; }

        // l -> r ... l -> M+1 のすべての区間が条件を満たす
        ans[r - l] += 1;
        ans[(M + 1) - l + 1] -= 1;

        // 区間から外す処理
        for &x in &inv[l] {
            cnt[x] -= 1;
            if cnt[x] == 0 {
                cnt_zero += 1;
            }
        }
    }

    // imos
    for i in 1..=M {
        ans[i] += ans[i - 1];
        print!("{} ", ans[i]);
    }
    println!();
}
