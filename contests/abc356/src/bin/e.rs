#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    let mut cnt: Vec<usize> = vec![0; SIZE + 10];

    for &a in &A {
        cnt[a] += 1;
    }

    for i in 0..SIZE {
        cnt[i + 1] += cnt[i];
    }

    let mut ans = 0_usize;

    for c in 1..SIZE {
        let d = cnt[c] - cnt[c - 1];

        for k in 1..(SIZE + c) / c {
            let kc = k * c;
            ans += k * (cnt[SIZE.min(kc + c - 1)] - cnt[kc - 1]) * d;
        }

        // i内での和
        ans -= d * (d + 1) / 2;
    }

    println!("{ans}");
}

const SIZE: usize = 1001001;

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
