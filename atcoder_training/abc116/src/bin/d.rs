#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        K: usize,
        TD: [(Usize1, usize); N]
    }

    // 美味しさの昇順にソート
    let mut succ = TD.iter().cloned().sorted_by_key(|&(t, d)| d).collect_vec();

    debug!(succ);

    // 貪欲にK個とる
    // 現在の種類数
    let mut p = 0;
    // 現在のスコア
    let mut score = 0;
    // 採用した寿司の種類ごとの個数
    let mut cnt = vec![0; N];
    // 採用した寿司（おいしさ，種類）
    let mut A = BinaryHeap::new();

    for _ in 0..K {
        // 現在の最大値の取り出し
        let (t, d) = succ.pop().unwrap();
        cnt[t] += 1;
        if cnt[t] == 1 {
            p += 1;
        }
        score += d;
        A.push(Reverse((d, t)));
    }

    // 種類を増やしながらスコアを調べる
    let mut ans = score + p * p;

    'outer: while let Some(Reverse((del_d, del_t))) = A.pop() {
        // 削除すると種類が減ってしまう場合 → 無視
        if cnt[del_t] == 1 {
            continue;
        }
        cnt[del_t] -= 1;
        score -= del_d;
        // 追加されていないものの中で，最もスコアが大きいもの
        while let Some((ins_t, ins_d)) = succ.pop() {
            // 追加しても種類が増えない → 無視
            if cnt[ins_t] > 0 {
                continue;
            }
            cnt[ins_t] += 1;
            score += ins_d;
            p += 1;
            // スコアの更新
            chmax! {
                ans,
                score + p * p
            };
            continue 'outer;
        }
        break;
    }

    println!("{ans}");
}

mod chmax {
    //! chmaxの実装
    /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
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
}
