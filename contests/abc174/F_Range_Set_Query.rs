//           F - Range Set Query
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc174/tasks/abc174_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

// imports
use itertools::Itertools;
use num_integer::Roots;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
        c: [Usize1; N],
        queries: [(Usize1, Usize1); Q],
    }

    // バケットの大きさ
    let BUCKET_RANGE = N / Q.sqrt() + 1;
    let BUCKET_CNT = N / BUCKET_RANGE + 1;

    // バケットに分割
    let mut buckets = vec![vec![]; BUCKET_CNT];

    for (i, &(l, r)) in queries.iter().enumerate() {
        buckets[l / BUCKET_RANGE].push((l, r, i))
    }

    // 区間をソート（↑↓↑↓）
    for i in 0..BUCKET_CNT {
        buckets[i].sort_by_key(|&(l, r, i)| r);
        if i % 2 == 1 {
            buckets[i].reverse();
        }
    }
    
    debug!(&buckets);


    // 答えを格納する配列
    let mut ans = vec![0_usize; Q];

    // 区間の処理
    let mut x = 0;  // 現在の区間の左端
    let mut y = 0;  // 現在の区間の右端
    let mut set = RangeSet::new(N);

    for i in 0..BUCKET_CNT {
        for &(l, r, i) in &buckets[i] {
            while y <= r {
                set.add(c[y]);
                y += 1;
            }
            while x > l {
                x -= 1;
                set.add(c[x]);
            }
            while y > r + 1 {
                y -= 1;
                set.del(c[y]);
            }
            while x < l {
                set.del(c[x]);
                x += 1;
            }
            ans[i] = set.count;
        }
    }

    println!("{}", ans.iter().join("\n"));
}

/// # RangeSet
/// - 区間に含まれる集合の種類を管理する
pub struct RangeSet {
    count: usize,
    data: Vec<usize>,
}

impl RangeSet {
    pub fn new(n: usize) -> Self {
        Self { count: 0, data: vec![0; n] }
    }

    pub fn add(&mut self, i: usize) {
        if self.data[i] == 0 {
            self.count += 1;
        }
        self.data[i] += 1;
    }

    pub fn del(&mut self, i: usize) {
        self.data[i] -= 1;
        if self.data[i] == 0 {
            self.count -= 1;
        }
    }
}
