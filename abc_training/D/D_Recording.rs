//              D - Recording              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc080/tasks/abc080_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        dbg!($($val),*);
    }};
}

fn print2d(arr: &Vec<Vec<isize>>) {
    for row in arr {
        let mut curr = 0;
        for &v in row {
            curr += v;
            if curr > 0 {
                print!("+");
            } else {
                print!("-");
            }
        }
        println!();
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 区間スケジューリング問題
fn main() {
    input! {
        N: usize,
        C: usize,
        mut P: [(usize, usize, Usize1); N],
    }

    {
        #![cfg(debug_assertions)]
        let mut arr = vec![vec![0; 100]; C];
        for &(s, t, c) in &P {
            arr[c][s] += 1;
            arr[c][t] -= 1;
        }
    
        print2d(&arr);
    }

    // いもす法
    const SIZ: usize = 101010;
    let mut imos = vec![vec![0; SIZ]; C];

    for &(s, t, c) in &P {
        imos[c][s - 1] += 1;
        imos[c][t] -= 1;
    }

    // 累積和
    for i in 0..SIZ - 1 {
        for c in 0..C {
            imos[c][i + 1] += imos[c][i];
        }
    }

    {
        #![cfg(debug_assertions)]
        for c in 0..C {
            println!("{:?}", &imos[c][0..50]);
        }
    }

    // ある時点における重複数
    let mut ans = 0;
    for i in 0..SIZ {
        let mut tmp = 0;
        for c in 0..C {
            tmp += imos[c][i].min(1);
        }
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}