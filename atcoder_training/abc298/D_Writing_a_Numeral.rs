//          D - Writing a Numeral
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc298/tasks/abc298_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd_assign {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 998_244_353;

// main
#[fastout]
fn main() {
    input! {
        Q: usize,
    }

    let mut nums = VecDeque::<usize>::new();
    nums.push_back(1);
    let mut res = 1;

    // query
    for _ in 0..Q {
        input! { q: usize }
        // dbg!(&nums);
        if q == 1 {
            input! { x: usize }
            nums.push_back(x);
            res = res.mmul(10).madd(x);
        } else if q == 2 {
            let x = nums.pop_front().unwrap();
            res = res.msub(x.mmul(10.mpow(nums.len())));
        } else if q == 3 {
            println!("{}", res.val());
        }
    }
}
