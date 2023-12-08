// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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
const UNST: &str = "UNRESTORABLE";

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

fn main() {
    input! {
        S: String,
        T: String
    }

    let sl = S.len();
    let tl = T.len();

    if sl < tl {
        println!("{}", UNST);
        return;
    }

    let mut ans = "あああああ".to_string();

    for i in 0..=sl - tl {
        // 判定
        let mut tmp = "?".repeat(sl);
        tmp.replace_range(i..i+tl, &T);

        debug!(tmp);

        if S.chars().zip(tmp.chars()).all(|(c, d)| c == '?' || d == '?' || c == d) {
            let mut tmp = S.clone();
            tmp.replace_range(i..i + tl, &T);
            chmin! {
                ans,
                tmp.replace("?", "a"),
            };
        }
    }

    if ans == "あああああ" {
        println!("{}", UNST);
    } else {
        println!("{}", ans);
    }
}
