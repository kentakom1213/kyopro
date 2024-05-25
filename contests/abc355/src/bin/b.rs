#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }

    // let res = A
    //     .iter()
    //     .map(|&a| (a, true))
    //     .(B.iter().map(|&b| (b, false)))
    //     .sorted()
    //     .collect_vec();

    let mut C = vec![];

    for &a in &A {
        C.push((a, true));
    }
    for &b in &B {
        C.push((b, false));
    }

    C.sort();

    let mut isadj = C[0].1 && C[1].1;

    for i in 1..N + M {
        isadj |= C[i - 1].1 && C[i].1;
    }

    if isadj {
        println!("Yes");
    } else {
        println!("No");
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
