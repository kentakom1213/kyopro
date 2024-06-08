#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: u32
    }

    let size = 3_usize.pow(N) as usize;
    let mut tile = vec![vec![true; size]; size];

    rec(0, 0, size, size, &mut tile);

    for row in tile {
        println!(
            "{}",
            row.iter().map(|x| if *x { '#' } else { '.' }).join("")
        );
    }
}

fn rec(r: usize, c: usize, k: usize, S: usize, tile: &mut Vec<Vec<bool>>) {
    debug!(r, c, k);
    if k == 1 {
        return;
    }
    // 中心を埋める
    let l = k / 3;
    for i in l..l + l {
        for j in l..l + l {
            tile[r + i][c + j] = false;
        }
    }
    // 再帰
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            rec(r + i * l, c + j * l, k / 3, S, tile);
        }
    }
}

const _INF: usize = 1001001001001001001;

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
