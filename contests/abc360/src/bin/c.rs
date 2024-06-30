#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
        W: [usize; N]
    }

    let mut boxs = vec![vec![]; N];

    for (&a, &w) in A.iter().zip(&W) {
        boxs[a].push(w);
    }

    let ans = boxs
        .iter()
        .map(|x| x.iter().sum::<usize>() - *x.iter().max().unwrap_or(&0))
        .sum::<usize>();

    println!("{ans}");
}

const INF: usize = 1001001001001001001;

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
