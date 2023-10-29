//                 D - 756                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc114/tasks/abc114_d
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

/// ## 方針
/// - 素因数の指数を考える
fn main() {
    input! {
        N: usize,
    }

    let mut cnt = vec![0; N + 1];  // 約数をカウント
    for n in 2..=N {
        for i in factorize(n) {
            cnt[i] += 1;
        }
    }

    // 答え
    let mut ans = 0;

    // a^2 * b^4 * c^4 => (2+1) * (4+1) * (4+1) = 75
    for i in 2..=N {
        for j in 2..=N {
            for k in j+1..=N {  // j < k
                if i != j && i != k {
                    let isOK = cnt[i] >= 2 && cnt[j] >= 4 && cnt[k] >= 4;
                    if isOK {
                        ans += 1;
                    }
                }
            }
        }
    }

    for i in 2..=N {
        for j in 2..=N {
            if i != j {
                // a^4 * b^14 => (4+1) * (14+1) = 75
                if cnt[i] >= 4 && cnt[j] >= 14 {
                    ans += 1;
                }

                // a^2 * b^24 => (2+1) * (24+1) = 75
                if cnt[i] >= 2 && cnt[j] >= 24 {
                    ans += 1;
                }
            }
        }
    }

    // a^74 => 74+1 = 75
    for i in 2..=N {
        if cnt[i] >= 74 {
            ans += 1;
        }
    }

    println!("{}", ans);
}

/// ## 素因数分解
fn factorize(mut N: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 2..N {
        if i * i > N { break; }
        while N % i == 0 {
            res.push(i);
            N /= i;
        }
    }
    if N > 1 {
        res.push(N);
    }
    res
}
