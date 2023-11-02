// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
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

// solve
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [Chars; H],
    }

    // 座標圧縮
    let mut rs = vec![];
    let mut cs = vec![];

    for i in 0..H {
        for j in 0..W {
            if A[i][j] == '#' {
                rs.push(i);
                cs.push(j);
            }
        }
    }

    // ソート
    rs.sort();
    cs.sort();

    // 重複の削除
    rs.dedup();
    cs.dedup();

    // 答え
    let mut ans = vec![vec!['.'; W]; H];
    let (mut rmax, mut cmax) = (0, 0);

    for i in 0..H {
        for j in 0..W {
            if A[i][j] == '#' {
                let r = rs.lower_bound(i);
                let c = cs.lower_bound(j);
                ans[r][c] = '#';
                chmax!(rmax, r);
                chmax!(cmax, c);
            }
        }
    }
    
    // 表示
    for i in 0..=rmax {
        for j in 0..=cmax {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}