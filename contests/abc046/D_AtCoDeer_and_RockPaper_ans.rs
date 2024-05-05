//       D - AtCoDeer and Rock-Paper
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc046/tasks/arc062_b
// ----------------------------------------

/*
            ...................                ...................
            .bbbbbkbbbbkbbbbbb:                .bbbbbbbbbbbbbbbbb:
        .....UUUUUUUUUUUUUUUUU:....        .....UUUUUUUUUUUUUUUUU:....
        (bbkR:~::~::~::~::~:~:dbbb]        (kkbR:~::~::~::~::~:~:dbbb]
        (bbbR:~:~:~:~:~:~:~::~dbbb]        (bbbR:~:~:~~:~~:~~:~:~dbbb]
    bbbbr~::~::~:~:~:~::~::~~:~::~(bbkkkkkkr:~::~:~:~::~::~::~::~::~:(bbkb
    bbbbr:~::~~::~::~:~:~~::~::~~:(bbbbbbbbr~:~:~::~::~::~::~:~::~:~:(bbbb
    bbbbr:~::~:~:dkkkkkkkkkkkkc::~(bbbbbbbbr:~:(kkkkkkkkkkkkk:~~:~::~(bbbb
    bbbbr~::~:~:~Xbbbbbbbbbbbbl~::(pbbbbbbpr:~:(bbbbbbbbbbbpK::~::~~:(pbbb
    """"1+J+J:~::7TTTTTTTTTTTTGJJJJbbbpbbbbnJJJJTTTTTTTTTTTTT~::~(JJJJ""""
        (bbbR~:~:~::~::::~::~:dbbbbbbbbbbbbbbbbR::~::~::~::~:~~:~dbbb]
        (UUUS(((((((((((((((((ZUUUWbbbbbbbbVUUUS(((((((((((((((((ZUUU%
            .bbbbbbpbbpbbbppbp:   .bbbpbbbb)   .pbbppbpppppbbbbbb:
            .pbbbbbbbbbbbbbbbp:   .bbbbbbbb)   .pbbbbbbbbbbbbbbbb:
                                  .bbbbbbbb)
                                  .bbbpbbbb)
                                  .bbbbbbbb)
                                  .bbbbbbbb)
                                  .bbbbpbbb)
                                  .bbbbbbbb)
                                  .bbbbbbbb)                          `
                                  .bbbbpbbb)
                                  .bbbbbbbb)       `   `   `    `   `
                                  .bbbbbbbb)    `
                                  .bbbbpbbb)
                     .NNNNNNNNNNNNNMMMMMMMMNNNNNNNNNNNNN]
                     .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM]    `    `   `
    .................(MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMh.................
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
                               ©︎ POWELL 2023
*/

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
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

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
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
const INF: isize = 1001001001001001001;

// main
fn main() {
    input! {
        S: String,
    }

    let N = S.len() as isize;  // 試合の回数
    let P = S.chars().filter(|&c| c == 'p').count() as isize;  // 相手がパーを出す回数

    let ans = N / 2 - P;
    println!("{}", ans);
}
