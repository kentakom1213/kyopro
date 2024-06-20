#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [String; N]
    }

    let T = S.iter().map(|s| {
        let mut bit = 0;
        s.chars().enumerate().for_each(|(i, c)| {if c == 'o' { bit |= 1 << i }});
        bit
    }).collect_vec();

    debug!(T);

    let mut ans = u32::MAX;

    for s in 0..1_u32 << N {
        let mut tmp = 0;
        for i in 0..N {
            if s >> i & 1 == 1 {
                tmp |= T[i];
            }
        }
        if tmp == (1 << M) - 1 {
            ans = ans.min(s.count_ones());
        }
    }

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
