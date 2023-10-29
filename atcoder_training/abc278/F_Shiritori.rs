//              F - Shiritori
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc278/tasks/abc278_f
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// - ゲーム問題はミニマックス法
fn main() {
    input! {
        N: usize,
        SS: [String; N],
    }

    let words: Vec<(char, char)> = SS
        .iter()
        .map(|s| (s.chars().next().unwrap(), s.chars().last().unwrap()))
        .collect();
    let mut memo = vec![vec![None; 1 << N]; N];

    let mut can_Taro = false;
    for i in 0..N {
        can_Taro |= mini_max(i, 1 << i, &words, &mut memo);
    }

    if can_Taro {
        println!("First");
    } else {
        println!("Second");
    }
}

/// ## mini_max
/// - 現在すでに使われている単語の集合が`S`
/// - 直前に使われた単語が`u`
/// で表現されるとき、
/// 残っている単語から最適な候補を選んだとき、太郎が勝つか否か
fn mini_max(
    u: usize,
    S: usize,
    words: &Vec<(char, char)>,
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    // メモ
    if let Some(res) = memo[u][S] {
        return res;
    }

    // 太郎の手番かどうか
    let is_Taro = S.count_ones() % 2 == 0;

    // 直前の単語の最後
    let prev = words[u].1;

    // 次に使える単語の集合
    let mut can_use = vec![];
    for (i, &(s, _)) in words.iter().enumerate() {
        if (S >> i) & 1 == 0 && s == prev {
            can_use.push(i);
        }
    }

    // 次に使える単語がないとき
    if can_use.is_empty() {
        memo[u][S] = Some(!is_Taro);
        return !is_Taro;
    }

    if is_Taro {
        // 次郎が取る手の評価値のうち、最大になる値を選択
        let mut best = false;
        for i in can_use {
            best |= mini_max(i, S | (1 << i), words, memo);
            memo[i][S | (1 << i)] = Some(best);
        }
        best
    } else {
        // 太郎が取る手の評価値のうち、最小になる値を選択
        let mut best = true;
        for i in can_use {
            best &= mini_max(i, S | (1 << i), words, memo);
            memo[i][S | (1 << i)] = Some(best);
        }
        best
    }
}
