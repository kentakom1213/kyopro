//            A - 119 × 2^23 + 1           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc119/tasks/arc119_a
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - 2^b <= N を満たす全てのbについて調べる
/// - a+cの最小値については3分探索
fn main() {
    input!{N: usize}

    // a, b の値から a*2^b+c == N を満たす(a,b,c)についてのa+b+cの値を求める
    let sum = |a: usize, b: usize| {
        if N < a * (1 << b) {
            INF
        }
        else {
            let c = N - a * (1 << b);
            a + b + c
        }
    };

    let mut ans = INF;

    let mut b = 0;
    while (1 << b) <= N {
        // aの値を3分探索
        let (mut l, mut r) = (0, INF / (1 << b));
        while (r - l) > 2 {
            let m1 = (2 * l + r) / 3;
            let m2 = (l + 2 * r) / 3;

            if sum(m1, b) <= sum(m2, b) {
                r = m2;
            }
            else {
                l = m1;
            }
        }
        let a = (l + r) / 2;

        // 答えの更新
        chmin!{
            ans,
            sum(a, b)
        };

        // bをインクリメント
        b += 1;
    }

    println!("{}", ans);
}
