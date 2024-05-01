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

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

const INF: isize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        K: Usize1,
        A: [isize; N]
    }

    // An := Aのうち，負であるもの
    // A0 := Aのうち，0であるもの
    // Ap := Aのうち，正であるもの
    let [An, A0, Ap] = A
        .iter()
        .sorted()
        .fold([vec![], vec![], vec![]], |mut v, &a| {
            if a < 0 {
                v[0].push(a);
            } else if a == 0 {
                v[1].push(a);
            } else {
                v[2].push(a);
            }
            v
        });

    // K番目が負,0,正のどれであるかを判定する
    let (nn, n0, np) = (An.len(), A0.len(), Ap.len());

    let Nn = nn * np;
    let N0 = n0 * (nn + np) + n0 * n0.saturating_sub(1) / 2;

    if K < Nn {
        // K番目が負のとき

        // 「x未満の数がK個以下」であるような最大のxを求める
        let mut ok = -INF;
        let mut ng = 0;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if count_neg(mid, &An, &Ap) <= K {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{ok}");
    } else if K < Nn + N0 {
        // K番目が0のとき
        println!("0");
    } else {
        // K番目が正のとき

        // 「x未満の数がK個以下」であるような最大のxを求める
        let mut ok = 0;
        let mut ng = INF;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if Nn + N0 + count_pos(mid, &An, &Ap) <= K {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{ok}");
    }
}

const NEG1: usize = 1_usize.wrapping_neg();

/// ペアの積が負になるもののうち，x未満の負の数の個数を調べる
fn count_neg(x: isize, An: &[isize], Ap: &[isize]) -> usize {
    let mut ans = 0;

    // 負の数 x 正の数
    for (i, &a) in An.iter().enumerate() {
        let mut ok = NEG1;
        let mut ng = Ap.len();
        while ng.wrapping_sub(ok) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            // Apの下からmid番目
            let idx = Ap.len() - mid - 1;
            if a * Ap[idx] < x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += ok.wrapping_add(1);
    }

    ans
}

/// ペアの積が正になるもののうち，x未満の正の数の個数を調べる
fn count_pos(x: isize, An: &[isize], Ap: &[isize]) -> usize {
    let mut ans = 0;

    // 負の数 x 負の数
    for i in 0..An.len() {
        let mut ok = NEG1;
        let mut ng = i;
        while ng.wrapping_sub(ok) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            let idx = An.len() - mid - 1;
            if An[An.len() - i - 1] * An[idx] < x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += ok.wrapping_add(1);
    }

    // 正の数 x 正の数
    for (i, &a) in Ap.iter().enumerate() {
        let mut ok = NEG1;
        let mut ng = i;
        while ng.wrapping_sub(ok) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            if a * Ap[mid] < x {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ans += ok.wrapping_add(1);
    }

    ans
}
