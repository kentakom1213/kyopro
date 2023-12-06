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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

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

fn main() {
    input! {
        N: usize,
    }

    let state = (0..N).fold(vec![], |mut s, _| {
        input! {
            a: usize,
            xy: [(Usize1, usize); a],
        }
        s.push(xy);
        s
    });

    // bit全探索
    let mut ans = 0;

    for i in 0..1_usize << N {
        // 正しいか検証
        let mut is_valid = true;
        for a in 0..N {
            // 不親切
            if (i >> a) & 1 == 0 {
                continue;
            }
            for &(x, y) in &state[a] {
                is_valid &= (i >> x) & 1 == y;
            }
        }
        if is_valid {
            chmax! {
                ans,
                i.count_ones() as usize
            };
        }
    }

    println!("{}", ans);
}
