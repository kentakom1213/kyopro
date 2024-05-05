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

/*
AAABCABCABCAABCABCBBBAABCBCCCAAABCBCBCC
AAABCAABCABCBBBAABCBCCCAAABCBCBCC
AAAABCABCBBBAABCBCCCAAABCBCBCC
AAAABCBBBAABCBCCCAAABCBCBCC
AAABBBAABCBCCCAAABCBCBCC
AAABBBABCCCAAABCBCBCC
AAABBBCCAAABCBCBCC
AAABBBCCAABCBCC
AAABBBCCABCC
AAABBBCCC
*/

fn main() {
    input! {
        S: String,
    }

    let mut st = vec![];

    for c in S.chars() {
        let k = st.len();
        if k >= 2 && st[k - 2] == 'A' && st[k - 1] == 'B' && c == 'C' {
            st.pop();
            st.pop();
        } else {
            st.push(c);
        }
    }

    println!("{}", st.iter().join(""));
}
