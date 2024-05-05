#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N]
    }

    // 偏角でソート
    let P = XY
        .iter()
        .map(|&(x, y)| (Frac(y, x - 1), Frac(y - 1, x)))
        .sorted()
        .collect_vec();

    debug!(P);

    // 区間スケジューリング
    let mut ans = 0;
    let mut cur = Frac(0, 1);

    for &(end, start) in &P {
        if cur <= start {
            ans += 1;
            cur = end;
        }
    }

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

use std::{cmp::Ordering, ops::Mul};

/// 分数を表す構造体
/// - `Frac(a, b)` := a / b
#[derive(Debug, Clone, Copy)]
pub struct Frac<T>(T, T);

impl<T> PartialEq for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn eq(&self, other: &Self) -> bool {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        a1 * b2 == a2 * b1
    }
}

impl<T> Eq for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> PartialOrd for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        (a1 * b2).partial_cmp(&(a2 * b1))
    }
}

impl<T> Ord for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
