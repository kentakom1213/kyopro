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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

const NEG1: usize = 1_usize.wrapping_neg();
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: Usize1,
        mut A: [isize; N]
    }

    if cfg!(debug_assertions) {
        let mut all = vec![];
        for i in 0..N {
            for j in 0..i {
                all.push(A[i] * A[j]);
            }
        }
        all.sort();
        eprintln!("{all:?}");
    }

    // Aをソート
    A.sort();

    // A[i] * A[j] がx未満である(i,j)の個数を数える
    let count = |x: isize| -> usize {
        let mut ans = 0;
        for i in 0..N {
            if A[i] >= 0 {
                let mut ok = NEG1;
                let mut ng = N;
                while ng.wrapping_sub(ok) > 1 {
                    let mid = ok.wrapping_add(ng) / 2;
                    if A[i] * A[mid] < x {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                if ok != NEG1 && i <= ok {
                    ans += ok;
                } else {
                    ans += ok.wrapping_add(1);
                }
            } else {
                let mut ok = NEG1;
                let mut ng = N;
                while ng.wrapping_sub(ok) > 1 {
                    let mid = ok.wrapping_add(ng) / 2;
                    if A[i] * A[N - mid - 1] < x {
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                if ok != NEG1 && N.wrapping_sub(ok) - 1 <= i {
                    ans += ok;
                } else {
                    ans += ok.wrapping_add(1);
                }
            }
        }
        ans / 2
    };

    let mut ok = -INF;
    let mut ng = INF;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        if count(mid) <= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
