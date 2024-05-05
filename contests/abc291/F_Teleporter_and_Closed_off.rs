//      F - Teleporter and Closed off      
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc291/tasks/abc291_f
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

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// solve
fn main() {
    input! {
        N: usize,
        M: usize,
        SS: [Chars; N],
    }

    let mut dp0 = vec![INF; N];
    let mut dp1 = vec![INF; N];
    dp0[0] = 0;
    dp1[N-1] = 0;
    for i in 0..N {
        for j in 0..M {
            if SS[i][j] == '1' {
                chmin!{
                    dp0[i+j+1],
                    dp0[i] + 1
                };
            }
        }
    }
    for i in (0..N).rev() {
        for j in (0..M).rev() {
            if SS[i][j] == '1' {
                chmin!{
                    dp1[i],
                    dp1[i+j+1] + 1
                };
            }
        }
    }

    // kを通らないで0からN-1に行くときの最短経路を求める
    for k in 1..N-1 {
        let mut tmp = INF;

        let start = k.saturating_sub(M);
        let end = (k + M).min(N);

        for i in start..end {
            for j in i+1..end {
                // 通れる場合
                if i < k && k < j && 2 <= j-i && j-i <= M && SS[i][j-i-1] == '1' {
                    tmp = tmp.min(
                        dp0[i] + dp1[j] + 1
                    );
                }
            }
        }

        if tmp < INF {
            print!("{} ", tmp);
        } else {
            print!("-1 ");
        }
    }
    println!();
}
