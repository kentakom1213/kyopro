// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use std::ops::{Add, Mul, Sub};
use std::{collections::HashMap, hash::Hash};

pub struct Loop<T, V, F, G>
where
    F: Fn(T) -> T,
    G: Fn(T) -> V,
{
    /// ノードの移動を行う関数
    pub next: F,
    /// ノードから値を取り出す関数
    pub get_val: G,
    /// 始点となるノード
    pub begin: T,
    /// ループの長さ
    pub loop_len: usize,
    /// ループ開始時の値
    pub loop_begin: T,
    /// ループに到達するまでの移動回数
    pub loop_begin_idx: usize,
    /// ループ開始時までの累積
    pub before_loop_sum: V,
    /// ループ内での累積
    pub loop_sum: V,
    /// ループの途中の値
    vals: HashMap<T, (usize, V)>,
}

impl<T, V, F, G> Loop<T, V, F, G>
where
    T: Copy + Hash + Eq,
    V: Copy + Zero + Add<Output = V> + Sub<Output = V> + Mul<usize, Output = V>,
    F: Fn(T) -> T,
    G: Fn(T) -> V,
{
    /// ループを検出する
    pub fn build(begin: T, next: F, get_val: G) -> Self {
        // 初期化
        let mut cur: T = begin;
        let mut idx: usize = 0;
        let mut sum: V = V::zero();
        let mut vals: HashMap<T, (usize, V)> = HashMap::new();

        // ループ検出
        while vals.get(&cur).is_none() {
            vals.insert(cur, (idx, sum));
            sum = sum + get_val(cur);
            cur = next(cur);
            idx += 1;
        }

        // ループの値を取り出す
        let loop_begin = cur;
        let (loop_begin_idx, before_loop_sum) = vals[&loop_begin];
        let loop_len = idx - loop_begin_idx;
        let loop_sum = sum - before_loop_sum;

        // 返す
        Self {
            next,
            get_val,
            begin,
            loop_len,
            loop_begin,
            loop_begin_idx,
            before_loop_sum,
            loop_sum,
            vals,
        }
    }

    fn accumulate(&self, begin: T, n: usize) -> (T, V) {
        let mut res = V::zero();
        let mut cur = begin;
        for _ in 0..n {
            res = res + (self.get_val)(cur);
            cur = (self.next)(cur);
        }
        (cur, res)
    }

    /// self.beginからn個後の頂点を取り出す
    pub fn get_nth_node_usize(&self, n: usize) -> T {
        if n < self.loop_begin_idx {
            self.accumulate(self.begin, n).0
        } else {
            let loop_rem = (n - self.loop_begin_idx) % self.loop_len;
            self.accumulate(self.loop_begin, loop_rem).0
        }
    }

    /// self.beginからn個後の値を取り出す
    pub fn get_nth_val_usize(&self, n: usize) -> V {
        if n < self.loop_begin_idx {
            self.accumulate(self.begin, n).1
        } else {
            let loop_rep = (n - self.loop_begin_idx) / self.loop_len;
            let loop_rem = (n - self.loop_begin_idx) % self.loop_len;
            self.before_loop_sum
                + self.loop_sum * loop_rep
                + self.accumulate(self.loop_begin, loop_rem).1
        }
    }

    /// self.beginからn個後の値を取り出す
    pub fn get_nth_node_biguint(&self, n: BigUint) -> T {
        let loop_begin_idx = BigUint::from_usize(self.loop_begin_idx).unwrap();
        if n < loop_begin_idx {
            let n_usize = n.to_usize().unwrap();
            self.accumulate(self.begin, n_usize).0
        } else {
            let loop_len = BigUint::from_usize(self.loop_len).unwrap();
            let loop_rem = (n - loop_begin_idx) % loop_len;
            let loop_rem = loop_rem.to_usize().unwrap();
            self.accumulate(self.loop_begin, loop_rem).0
        }
    }
}

fn main() {
    input! {
        N: usize,
        a: Usize1,
        k: BigUint,
        B: [Usize1; N],
    }

    let g = Loop::build(a, |x| B[x], |x| 0);

    let ans = g.get_nth_node_biguint(k);

    println!("{}", ans + 1);
}
