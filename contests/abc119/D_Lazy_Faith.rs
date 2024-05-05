//              D - Lazy Faith             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc119/tasks/abc119_d
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
use std::ops::Bound::{Included, Excluded, Unbounded};

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

/// # maxマクロ
macro_rules! min {
    ( $n:expr $(,)* ) => {{
        $n
    }};
    ( $n:expr, $m:expr $(,)* ) => {{
        ($n).min($m)
    }};
    ( $n:expr, $( $m:expr ),+ $(,)* ) => {{
        ($n).min( min!( $( $m ),+ ) )
    }};
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
const INF: isize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # 方針
/// - 神社、寺について2個前から2個先まで見る
/// 
/// ```non-rust-code
///  t1 s1 t2   t3 s2 t3
/// ─┴─┴┴──┴─x──┴──┴──┴─
///     ┌────┘
/// ```
/// 
/// # 解説
/// - 1つ前後をみる
/// - 番兵を用意して処理を簡潔に
fn main() {
    let (A, B, Q) = get!(usize, usize, usize);
    let mut S = vec![-INF]; S.append(&mut get!(isize; A)); S.append(&mut vec![INF]);
    let mut T = vec![-INF]; T.append(&mut get!(isize; B)); T.append(&mut vec![INF]);
    let queries = get!(isize; Q);

    // クエリ処理
    for &x in &queries {
        let lbs = S.upper_bound(x);
        let lbt = T.upper_bound(x);

        let mut ans = INF;
        for s in vec![S[lbs-1], S[lbs]] {
            for t in vec![T[lbt-1], T[lbt]] {
                ans = min!(
                    ans,
                    (t - x).abs() + (s - t).abs(),
                    (s - x).abs() + (t - s).abs(),
                );
            }
        }
        println!("{}", ans);
    }
}
