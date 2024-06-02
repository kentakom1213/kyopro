#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: isize,
        mut A: [isize; N]
    }

    let isok = |A: &Vec<isize>| {
        let mut s = 0;
        let mut is_over_K = s >= K;

        for &a in A {
            s += a;
            if is_over_K && s < K {
                return false;
            }
            is_over_K |= s >= K;
        }
        true
    };

    // 負の数，正の数
    let (neg, pos): (Vec<isize>, Vec<isize>) = A.iter().partition(|&&x| x < 0);

    let mut arr = neg.iter().chain(&pos).cloned().collect_vec();

    if isok(&arr) {
        println!("Yes");
        println!("{}", arr.iter().join(" "));
        return;
    }

    arr.reverse();

    if isok(&arr) {
        println!("Yes");
        println!("{}", arr.iter().join(" "));
        return;
    }

    println!("No");
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
