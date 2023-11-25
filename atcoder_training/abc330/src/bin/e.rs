// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [isize; N],
        queries: [(Usize1, isize); Q]
    }

    let mut counter = FxHashMap::default();
    let mut mexset = MexSet::new();

    for &a in &A {
        *counter.entry(a).or_insert(0) += 1;
        mexset.insert(a);
    }

    for &(i, nxt) in &queries {
        let cur = A[i];
        A[i] = nxt;
        
        *counter.entry(nxt).or_insert(0) += 1;

        let cur_cnt = counter.entry(cur).or_insert(0);
        *cur_cnt -= 1;

        if *cur_cnt == 0 {
            mexset.delete(cur);
            debug!(cur, mexset);
        }

        mexset.insert(nxt);
        debug!(nxt, mexset);

        println!("{}", mexset.mex(0));
    }
}

use std::collections::BTreeSet;
/// 集合とそのmexを管理する
#[derive(Debug)]
pub struct MexSet {
    pub ranges: BTreeSet<(isize, isize)>,
}
impl MexSet {
    /// MexSetを初期化する
    pub fn new() -> Self {
        let ranges = [(isize::MIN, isize::MIN), (isize::MAX, isize::MAX)]
            .into_iter()
            .collect();
        Self { ranges }
    }
    /// 集合に要素`x`を追加する
    /// ### 戻り値
    /// - `true`: `x`が追加された場合
    /// - `false`: `x`がすでに存在していた場合
    pub fn insert(&mut self, x: isize) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        let &(r, rr) = self.ranges.range((x + 1, x + 1)..).next().unwrap();
        if x <= l {
            return false;
        }
        match (l == x - 1, x + 1 == r) {
            (false, false) => {
                self.ranges.insert((x, x));
            }
            (false, true) => {
                self.ranges.remove(&(r, rr));
                self.ranges.insert((x, rr));
            }
            (true, false) => {
                self.ranges.remove(&(ll, l));
                self.ranges.insert((ll, x));
            }
            (true, true) => {
                self.ranges.remove(&(ll, l));
                self.ranges.remove(&(r, rr));
                self.ranges.insert((ll, rr));
            }
        }
        true
    }
    /// 集合から要素`x`を削除する
    /// ### 戻り値
    /// - `true`: `x`が削除された場合
    /// - `false`: `x`がすでに存在していなかった場合
    pub fn delete(&mut self, x: isize) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if l < x {
            return false;
        }
        self.ranges.remove(&(ll, l));
        match (ll == x, x == l) {
            (false, false) => {
                self.ranges.insert((ll, x - 1));
                self.ranges.insert((x + 1, l));
            }
            (false, true) => {
                self.ranges.insert((ll, x - 1));
            }
            (true, false) => {
                self.ranges.insert((x + 1, l));
            }
            (true, true) => (),
        }
        true
    }
    /// **集合に含まれない**`x`以上で最小の整数を調べる
    pub fn mex(&self, x: isize) -> isize {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if ll <= x && x <= l {
            l + 1
        } else {
            x
        }
    }
}
