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

use crate::acc2d::acc2D;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [[usize; N]; N]
    }

    // 中央値がx以下である K x K 領域が存在するか
    let isok = |x: usize| -> bool {
        // x以上の要素は1
        let mut X = vec![vec![0; N]; N];
        for i in 0..N {
            for j in 0..N {
                if A[i][j] <= x {
                    X[i][j] = 1;
                }
            }
        }
        // 二次元累積和
        let S = acc2D(&X);
        // すべての領域を探索
        let L = N - K + 1;
        for t in 0..L {
            for l in 0..L {
                // x以上の要素が K*K/2 個以上存在する <=> 中央値がx以上
                debug!(x, t, l, S(t, t + K, l, l + K));
                if S(t, t + K, l, l + K) >= (K * K + 1) / 2 {
                    return true;
                }
            }
        }
        false
    };

    if cfg!(debug_assertions) {
        for x in 0..20 {
            eprintln!("x:{} -> {}", x, isok(x));
        }
    }

    // 解の存在性で二分探索
    let mut ng = 1_usize.wrapping_neg();
    let mut ok = INF;

    while ok.wrapping_sub(ng) > 1 {
        let mid = ng.wrapping_add(ok) / 2;

        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

const INF: usize = 1001001001;

mod acc2d {
    //! 2次元累積和
    use num_traits::Num;
    /// ## acc2D
    /// - 2次元累積和を取る
    /// ### 戻り値
    /// - `|r_start, r_end, c_start, c_end|: (usize, usize, usize, usize) -> T`
    pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
        let (H, W) = (array.len(), array[0].len());
        let mut S = vec![vec![T::zero(); W + 1]; H + 1];
        for i in 0..H {
            for j in 0..W {
                S[i + 1][j + 1] = array[i][j] + S[i][j + 1] + S[i + 1][j] - S[i][j];
            }
        }
        move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
            S[r_end][c_end] + S[r_start][c_start] - S[r_end][c_start] - S[r_start][c_end]
        }
    }
}
