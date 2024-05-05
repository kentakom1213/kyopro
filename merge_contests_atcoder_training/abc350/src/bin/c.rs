#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::{sorted, Itertools};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        mut A: [Usize1; N],
    }

    // 逆引き辞書
    let mut invA = vec![0; N];
    for (i, &a) in A.iter().enumerate() {
        invA[a] = i;
    }

    debug!(A, invA);

    let mut ans = vec![];

    for i in 0..N {
        if invA[i] == i {
            continue;
        }
        // 入れ替える
        let old = invA[i];
        A.swap(i, old);
        ans.push((i, old));
        invA[A[old]] = old;

        debug!(A, invA);
    }

    println!("{}", ans.len());
    for &(a, b) in &ans {
        println!("{} {}", a + 1, b + 1);
    }
}
