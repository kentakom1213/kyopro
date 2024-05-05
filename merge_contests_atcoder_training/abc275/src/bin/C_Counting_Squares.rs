//           C - Counting Squares          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc275/tasks/abc275_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::num::Wrapping;
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;


// solve
fn main() {
    let S: Vec<Vec<char>> = get!(String; 9).iter().map(|s| s.chars().collect()).collect();

    // チェス盤上の2点を列挙
    let mut ans = 0;
    for ar in 0..9_usize {
        for ac in 0..9 {
            for br in ar+1..9 {
                for bc in 0..=ac {
                    let (cr, cc) = (
                        ar + ac - bc,  // ar - diff_c
                        ac + br - ar,  // ac + diff_r
                    );

                    let (dr, dc) = (
                        br + ac - bc,  // br - diff_c
                        bc + br - ar,  // bc + diff_r
                    );

                    if cr < 9 && cc < 9 && dr < 9 && dc < 9 {
                        let isOK = S[ar][ac] == '#'
                                && S[br][bc] == '#'
                                && S[cr][cc] == '#'
                                && S[dr][dc] == '#';
                        if isOK {
                            eprintln!(
                                "({}, {}), ({}, {}), ({}, {}), ({}, {})",
                                ar, ac,
                                br, bc,
                                cr, cc,
                                dr, dc,
                            );
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
