//         081 - Friendly Group（★5）
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/typical90/tasks/typical90_cc
// ----------------------------------------

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
use std::collections::VecDeque;

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

fn main() {
    input! {
        N: usize,
        K: usize,
        mut AB: [(usize, usize); N],
    }

    // 身長でソート
    AB.sort();

    const SIZE: usize = 5050;

    // 体重iの人の人數を管理するセグ木
    let mut cnt = vec![0; SIZE];

    // 尺取り法を管理するdeq
    let mut deq = VecDeque::new();

    // 尺取り法
    let mut ans = 0;

    for &(a, b) in &AB {
        // 要素を1つ追加
        deq.push_back((a, b));
        cnt[b] += 1;

        // 区間に含まれるメンバーの身長の差の絶対値がK以下になるように調整
        while let Some(&(ta, tb)) = deq.front() {
            if ta + K < a {
                deq.pop_front();
                cnt[tb] -= 1;
            } else {
                break;
            }
        }

        if cfg!(debug_assertions) {
            println!("{:?}", &cnt[..20]);
        }

        // ----- 条件を満たす区間 -----
        let mut tmp: usize = cnt[..=K].iter().sum();
        for i in 0..SIZE - K - 1 {
            chmax!(ans, tmp);
            // 区間の更新
            tmp -= cnt[i];
            tmp += cnt[i + K + 1];
        }
    }

    println!("{ans}");
}
