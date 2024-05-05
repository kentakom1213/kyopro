// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;

// imports
use itertools::Itertools;
use num_integer::gcd;
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

fn main() {
    input! {
        N: usize,
        K: usize,
        XY: [(isize, isize); N]
    }

    // 点がK個以上重なっている場合，infinity
    if XY
        .iter()
        .fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .values()
        .any(|&v| v >= K)
    {
        println!("Infinity");
        return;
    }

    // 直線を列挙
    let mut lines = HashMap::<(Frac, Frac), usize>::new();

    for i in 0..N {
        for j in i + 1..N {
            let (ax, ay) = XY[i];
            let (bx, by) = XY[j];
            let grad = Frac::new(ay - by, ax - bx);
            let mut sect = Frac::new(ax * by - ay * bx, ax - bx);

            if grad == Frac(0, 0) {
                sect = Frac(ax, 1);
            }

            *lines.entry((grad, sect)).or_insert(0) += 1;
        }
    }

    debug!(lines);

    // nC2の逆関数テーブル
    let comb_inv = (1..400)
        .map(|i| (i * (i - 1) / 2, i))
        .collect::<HashMap<usize, usize>>();

    // 何個の点が直線上を通ったかをカウント
    let ans = lines.values().filter(|v| comb_inv[v] >= K).count();

    println!("{}", ans);
}

/// 分数
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Frac(isize, isize);

impl Frac {
    #[inline]
    fn gcd(a: isize, b: isize) -> isize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }

    /// 分数 `a / b` を作成する
    pub fn new(a: isize, b: isize) -> Self {
        let sgn = a.signum() * b.signum();
        let (a, b) = (a.abs(), b.abs());
        let gcd = Self::gcd(a, b);
        if b == 0 {
            Self(0, 0)
        } else if a == 0 {
            Self(0, 1)
        } else {
            Self(sgn * a / gcd, b / gcd)
        }
    }
}
