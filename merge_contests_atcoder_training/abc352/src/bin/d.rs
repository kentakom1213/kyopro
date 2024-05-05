#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

use itertools::Itertools;
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
        K: usize,
        P: [Usize1; N]
    }

    // 逆の索引
    let IP = {
        let mut idx = vec![0; N];
        for i in 0..N {
            idx[P[i]] = i;
        }
        idx
    };

    let mut set = BTreeSet::new();

    // K個追加
    for i in 0..K {
        set.insert(IP[i]);
    }

    // 初期値
    let mut ans = set.last().unwrap() - set.first().unwrap();

    debug!(set);

    // sliding window
    for i in 0..N - K {
        // 末尾を削除
        set.remove(&IP[i]);
        // 先頭を追加
        set.insert(IP[i + K]);

        chmin! {
            ans,
            set.last().unwrap() - set.first().unwrap()
        };

        debug!(set);
    }

    println!("{ans}");
}

mod macro_chmin {
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
}
