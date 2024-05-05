//            D - Money in Hand            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc286/tasks/abc286_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        (N, X): (usize, usize),
        AB: [(usize, usize); N],
    }

    // dp[i][j] := i個目までの硬貨を用いて金額jを表現できるか
    let mut dp = vec![vec![0; 10101]; 55];
    dp[0][0] = 1;

    for (i, &(a, b)) in AB.iter().enumerate() {
        for j in 0..=X {
            if dp[i][j] == 0 { continue; }

            // b個ずつ加算していく
            for k in 0..=b {
                if j + a*k <= X {
                    dp[i+1][j + a*k] = 1;
                }
            }
        }
    }

    // for i in 0..=N {
    //     println!("{:?}", &dp[i][..=X]);
    // }

    if dp[N][X] == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
