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
use superslice::Ext;

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
        K: usize,
        A: [usize; N],
    }

    let mut kth = vec![INF; K + 1];
    kth[0] = 0;

    for k in 1..=K {
        for &a in &A {
            // a + x > kth[k - 1] となるような最小のxを見つける
            let mut ok = K + 1;
            let mut ng = 1_usize.wrapping_neg();
            while ok.wrapping_sub(ng) > 1 {
                let mid = ng.wrapping_add(ok) / 2;
                if a + kth[mid] > kth[k - 1] {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            if kth[k - 1] < kth[ok] + a && kth[ok] + a < kth[k] {
                kth[k] = kth[ok] + a;
            }
        }
    }

    println!("{}", kth[K]);
}
