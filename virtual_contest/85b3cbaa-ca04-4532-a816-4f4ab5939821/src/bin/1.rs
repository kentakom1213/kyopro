#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::cmp::Reverse;

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
        S: Chars,
        T: Chars
    }

    let (N, M) = (S.len(), T.len());

    if N < M {
        println!("{}", UNRESTORABLE);
        return;
    }

    let mut ans = None;

    'outer: for d in 0..N - M + 1 {
        let mut tmp = String::new();

        for i in 0..N {
            if d <= i && i < d + M {
                if S[i] == '?' || S[i] == T[i - d] {
                    tmp.push(T[i - d]);
                } else {
                    continue 'outer;
                }
            } else {
                if S[i] == '?' {
                    tmp.push('a');
                } else {
                    tmp.push(S[i]);
                }
            }
        }

        if ans.is_none() {
            ans = Some(tmp);
        } else {
            if ans.as_ref().unwrap() > &tmp {
                ans = Some(tmp);
            }
        }
    }

    if let Some(val) = ans {
        println!("{val}");
    } else {
        println!("{}", UNRESTORABLE);
    }
}

const UNRESTORABLE: &str = "UNRESTORABLE";

mod macro_chmin {
    #![allow(dead_code)]
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
