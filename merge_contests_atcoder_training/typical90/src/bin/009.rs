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
use std::f64::consts::PI;
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        XY: [(f64, f64); N]
    }

    let mut ans = 0.0;

    // 中央の点を全探索
    for b in 0..N {
        // 偏角ソート
        let points = (0..N)
            .filter(|&i| i != b)
            .map(|i| (arg(XY[b], XY[i])))
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect_vec();

        for &a in &points {
            // (a + 180) 以上の最小の値
            let target = (a + 180.0) % 360.0;
            let pos1 = points.lower_bound_by(|&c| c.partial_cmp(&target).unwrap());

            let c1 = pos1 % points.len();
            let c2 = (pos1 + points.len() - 1) % points.len();

            chmax! {
                ans,
                arg_diff(a, points[c1]),
                arg_diff(a, points[c2])
            };
        }
    }

    println!("{:.20}", ans);
}

/// `a`から見た`b`の偏角（0 ≦ θ < 360）
pub fn arg(a: (f64, f64), b: (f64, f64)) -> f64 {
    let theta = (b.1 - a.1).atan2(b.0 - a.0) * 180.0 / PI;
    (theta + 360.0) % 360.0
}

/// 偏角の差を求める（0 ≦ a - b < 180）
pub fn arg_diff(a: f64, b: f64) -> f64 {
    (a - b).abs().min(360.0 - (a - b).abs())
}
