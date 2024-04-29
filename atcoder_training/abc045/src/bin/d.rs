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

use itertools::{iproduct, Itertools};
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
        AB: [(Usize1, Usize1); N],
    }

    // マスをsetに格納
    let filled = AB.iter().cloned().collect::<FxHashSet<(usize, usize)>>();

    // 走査するマスの合計
    let mut total = (H - 3 + 1) * (W - 3 + 1);

    // 調べた3x3マスの左上の座標
    let mut scaned = FxHashSet::<(usize, usize)>::default();

    // 答えを格納する配列
    let mut ans = [0_usize; 10];

    let count = |t: usize, l: usize| -> usize {
        // 3x3の領域にある黒マスの数を数える
        iproduct!(0..3, 0..3)
            .filter(|&(dr, dc)| filled.contains(&(t + dr, l + dc)))
            .count()
    };

    // 色を塗られたマスの上下左右3マスを探索
    for &(r, c) in &filled {
        for &dr in &ADJ {
            for &dc in &ADJ {
                let t = r.wrapping_add(dr);
                let l = c.wrapping_add(dc);
                // 探索範囲外|探索済みは除く
                if t.saturating_add(2) >= H || l.saturating_add(2) >= W || scaned.contains(&(t, l))
                {
                    continue;
                }
                let cnt = count(t, l);
                debug!((t, l), cnt, total);
                ans[cnt] += 1;
                total -= 1;
                // すでに探索済み
                scaned.insert((t, l));
            }
        }
    }

    // その他は0
    ans[0] += total;

    println!("{}", ans.iter().join("\n"));
}

const INF: usize = 1001001001001001001;
const ADJ: [usize; 5] = [2_usize.wrapping_neg(), 1_usize.wrapping_neg(), 0, 1, 2];
