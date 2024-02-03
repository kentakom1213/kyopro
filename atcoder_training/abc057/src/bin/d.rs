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

use std::cmp::Reverse;

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        mut V: [isize; N],
    }

    // 逆順にソートし，平均の最大値を求める
    V.sort_by_key(|&x| Reverse(x));

    let max = {
        let mut cur = Frac(0, 0);
        let mut max = Frac(0, 1);

        for &a in &V {
            let Frac(sum, n) = cur;
            let nxt = Frac(sum + a, n + 1);
            if A as isize <= n + 1 && n + 1 <= B as isize {
                chmax! {
                    max,
                    nxt,
                };
            }
            cur = nxt;
            debug!(cur);
        }

        max
    };

    {
        let Frac(a, b) = max;
        println!("{:.10}", a as f64 / b as f64);
    }

    // 個数の復元
    if V[0] == V[A - 1] {
        // すべてV[0]を選ぶ場合に，平均が最大
        let X = V.iter().filter(|&&v| v == V[0]).count();
        let ans = (A..=B).map(|i| comb(X, i)).sum::<usize>();
        println!("{ans}");
    } else {
        // V[A-1]のえらびかた
        let X = V.iter().filter(|&&v| v == V[A - 1]).count();
        let Y = V[..A].iter().filter(|&&v| v == V[A - 1]).count();
        let ans = comb(X, Y);
        println!("{ans}");
    }
}

const INF: isize = 1001001001001001001;

use num_traits::Zero;
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
// TODO: Add, Mul等の実装

pub fn comb(n: usize, r: usize) -> usize {
    if r == 0 {
        1
    } else if n < r {
        0
    } else {
        n * comb(n - 1, r - 1) / r
    }
}
