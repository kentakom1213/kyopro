// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

// imports
use itertools::{merge, Itertools};
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        K: isize,
        XY: [(isize, isize); N]
    }

    // x,y軸独立に管理
    let mut X = vec![];
    let mut Y = vec![];
    let mut sx = 0;
    let mut sy = 0;

    for &(x, y) in &XY {
        X.push(x);
        Y.push(y);
        sx += x;
        sy += y;
    }

    // ソート
    X.sort();
    Y.sort();

    // 重心
    let gx = (sx as f64) / (N as f64);
    let gy = (sy as f64) / (N as f64);

    debug!(gx, gy);

    // 二分探索
    let mut ng = -1;
    let mut ok = INF;
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        let cost_x = calc_cost(mid, &X);
        let cost_y = calc_cost(mid, &Y);
        if cost_x + cost_y <= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

const INF: isize = 1001001001001001001;

/// 区間widthに対して，コストの最小値を計算
fn calc_cost(width: isize, points: &Vec<isize>) -> isize {
    // 最適な区間を調べる
    let N = points.len();
    let l = merge(points.iter().cloned(), points.iter().map(|&x| x - width))
        .nth(N - 1)
        .unwrap();
    // 区間を [l, l + width] に設定したときのコスト
    points
        .iter()
        .map(|&i| {
            if i < l {
                l - i
            } else if i <= l + width {
                0
            } else {
                i - (l + width)
            }
        })
        .sum::<isize>()
}
