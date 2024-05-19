#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input!(T: usize);

    for _ in 0..T {
        input! {
            N: usize,
            mut A: [isize; N]
        }

        let (lisl, L) = get_lis(&A);
        A.reverse();
        A.iter_mut().for_each(|x| *x *= -1);
        let (lisr, _) = get_lis(&A);

        debug!(lisl, lisr);

        // それぞれのLISを見る
        let mut ans = vec![];

        for i in 0..N {
            // LISになる場合
            if lisl[i] + lisr[N - i - 1] - 1 == L {
                ans.push(i + 1);
            }
        }

        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
    }
}

/// 配列Aの各要素について，その要素を右端とするLISの長さを求める
fn get_lis(A: &[isize]) -> (Vec<usize>, usize) {
    let N = A.len();
    let mut dp = vec![INF; N];
    let mut lis = vec![0; N];
    let mut max = 0;

    for (i, &a) in A.iter().enumerate() {
        let idx = dp.lower_bound(&a);
        dp[idx] = a;
        lis[i] = idx + 1;
        max = max.max(idx + 1);
        debug!(dp);
    }

    (lis, max)
}

const INF: isize = 1001001001001001001;

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
