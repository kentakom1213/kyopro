//           E - Prefix Equality           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc250/tasks/abc250_e
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

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v <= self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v < self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
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
    let B = get!(usize;;);
    let Q = get!(usize);
    let queries = get!(usize, usize; Q);

    // i番目の要素における先頭集合の要素数
    let mut kind_a = BTreeSet::new();
    let mut kind_b = BTreeSet::new();
    let mut cnt_a = vec![0; N+1];
    let mut cnt_b = vec![0; N+1];
    for i in 0..N {
        if kind_a.contains(&A[i]) {
            cnt_a[i+1] = cnt_a[i];
        } else {
            cnt_a[i+1] = cnt_a[i] + 1;
            kind_a.insert(&A[i]);
        }
        if kind_b.contains(&B[i]) {
            cnt_b[i+1] = cnt_b[i];
        } else {
            cnt_b[i+1] = cnt_b[i] + 1;
            kind_b.insert(&B[i]);
        }
    }

    let mut diff_ab = BTreeSet::new();
    let mut is_same = vec![false; N+1];
    for k in 1..=N {
        let i = cnt_a.lower_bound(k) - 1;
        let j = cnt_b.lower_bound(k) - 1;

        if i < N && !diff_ab.remove(&A[i]) {
            diff_ab.insert(&A[i]);
        }
        if j < N && !diff_ab.remove(&B[j]) {
            diff_ab.insert(&B[j]);
        }
        is_same[cnt_a[i]] = diff_ab.len() == 0;
    }

    // クエリ処理
    for &(x, y) in &queries {
        if cnt_a[x] == cnt_b[y] && is_same[cnt_a[x]] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
