#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        HT: [(char, Usize1); Q]
    }

    let mut cost = 0;
    let mut l = 0;
    let mut r = 1;

    for &(h, t) in &HT {
        if h == 'L' {
            cost += move_hand(l, t, r, N);
            l = t;
        } else {
            cost += move_hand(r, t, l, N);
            r = t;
        }
    }

    println!("{cost}");
}

/// 手を移動させる際のコストを計算する
///
/// - `s`: 移動する手の位置
/// - `t`: 目標となる手の位置
/// - `x`: 固定する手の位置
/// - `N`: 周の長さ
fn move_hand(mut s: usize, mut t: usize, mut x: usize, N: usize) -> usize {
    s %= N;
    t %= N;
    x %= N;

    // 右回転で a → b に移動するコスト
    let rot = |a: usize, b: usize| (b + N - a) % N;

    let mut sorted = [s, t, x];
    sorted.sort();
    match sorted {
        v if v == [s, t, x] => rot(s, t),
        v if v == [s, x, t] => rot(t, s),
        v if v == [x, s, t] => rot(s, t),
        v if v == [x, t, s] => rot(t, s),
        v if v == [t, x, s] => rot(s, t),
        v if v == [t, s, x] => rot(t, s),
        _ => unreachable!(),
    }
}
