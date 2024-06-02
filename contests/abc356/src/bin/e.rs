#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    let mut cnt = vec![0; SIZE];

    for &a in &A {
        cnt[a] += 1;

    }

    // 累積和
    let mut S = vec![0; SIZE + 1];

    for i in 0..SIZE {
        S[i + 1] = S[i] + cnt[i];
    }

    let mut ans = 0_usize;

    for i in 0..SIZE {
        for j in 1.. {
            if i * j >= SIZE {
                break;
            }

        }
    }

    println!("{ans}");
}

const SIZE: usize = 1001001;
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
