#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    let mut A = vec![];
    let mut R = vec![];

    for _ in 0..M {
        input! {
            c: usize,
            a: [Usize1; c],
            r: char
        }
        A.push(a);
        R.push(r == 'o');
    }

    let mut ans = 0;

    // bit全探索
    for S in 0..1 << N {
        let mut isok = true;
        // すべての選択肢を満たすか
        for j in 0..M {
            // 満たすものの個数
            let cnt = A[j].iter().filter(|&&a| S >> a & 1 == 1).count();
            isok &= R[j] == (cnt >= K);
        }

        if isok && cfg!(debug_assertions) {
            eprintln!("{:b}", S);
        }

        if isok {
            ans += 1;
        }
    }

    println!("{}", ans);
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
