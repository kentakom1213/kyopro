#![allow(non_snake_case)]

use proconio::input;

use crate::acc2d_cyclic::Acc2D;

fn main() {
    input! {
        N: usize,
        K: usize,
        XYC: [(usize, usize, char); N]
    }

    // 黒いブロックの数
    let mut B = vec![vec![0_u16; 2 * K]; 2 * K];
    // 白いブロックの数
    let mut W = vec![vec![0_u16; 2 * K]; 2 * K];

    for &(x, y, c) in &XYC {
        let mx = x % (2 * K);
        let my = y % (2 * K);

        if c == 'B' {
            B[mx][my] += 1;
        } else {
            W[mx][my] += 1;
        }
    }

    debug2D!(B);
    debug2D!(W);

    let Sb = Acc2D::new(&B);
    let Sw = Acc2D::new(&W);

    debug!(Sw.sum(.., ..), Sw.sum(..3, 3..));

    let mut ans = 0;

    for i in 0..2 * K {
        for j in 0..2 * K {
            // 1つ目の黒
            let b1 = Sb.sum_cyclic(i, j, K, K);
            // 2つ目の黒
            let b2 = Sb.sum_cyclic(i + K, j + K, K, K);
            // 1つ目の白
            let w1 = Sw.sum_cyclic(i, j + K, K, K);
            // 2つ目の白
            let w2 = Sw.sum_cyclic(i + K, j, K, K);

            debug!(i, j, b1, b2, w1, w2);

            let tmp = b1 + b2 + w1 + w2;

            chmax! {
                ans,
                tmp,
            };
        }
    }

    println!("{ans}");
}

mod acc2d_cyclic {
    use std::{
        fmt::Debug,
        ops::{
            Bound::{Excluded, Included, Unbounded},
            Mul, RangeBounds,
        },
    };

    use num_traits::Num;

    /// 2次元累積和
    pub struct Acc2D<T: Num + Copy> {
        pub H: usize,
        pub W: usize,
        pub S: Vec<Vec<T>>,
    }

    impl<T> Acc2D<T>
    where
        T: Num + Copy + TryFrom<usize> + Mul,
        <T as TryFrom<usize>>::Error: Debug,
    {
        #[inline]
        fn parse_range<R: RangeBounds<usize>>(
            &self,
            range: &R,
            max: usize,
        ) -> Option<(usize, usize)> {
            let start = match range.start_bound() {
                Unbounded => 0,
                Excluded(&v) => v + 1,
                Included(&v) => v,
            };
            let end = match range.end_bound() {
                Unbounded => max,
                Excluded(&v) => v,
                Included(&v) => v + 1,
            };
            if start <= end && end <= max {
                Some((start, end))
            } else {
                None
            }
        }

        /// 2次元配列から累積和を初期化する
        pub fn new(array: &Vec<Vec<T>>) -> Self {
            let (H, W) = (array.len(), array[0].len());
            let mut S = vec![vec![T::zero(); W + 1]; H + 1];
            for i in 0..H {
                for j in 0..W {
                    S[i + 1][j + 1] = array[i][j] + S[i][j + 1] + S[i + 1][j] - S[i][j];
                }
            }
            Self { H, W, S }
        }

        /// 累積和の値を求める
        pub fn sum<R, C>(&self, row: R, col: C) -> T
        where
            R: RangeBounds<usize> + Debug,
            C: RangeBounds<usize> + Debug,
        {
            let Some((rs, re)) = self.parse_range(&row, self.H) else {
                panic!("The given range is wrong (row): {:?}", row);
            };
            let Some((cs, ce)) = self.parse_range(&col, self.W) else {
                panic!("The given range is wrong (col): {:?}", col);
            };
            self.S[re][ce] + self.S[rs][cs] - self.S[re][cs] - self.S[rs][ce]
        }

        /// トーラスとみなしたときの和を求める
        /// ## Args
        /// - `(top,left)`：左上の座標
        /// - `(height,width)`：取得する区間
        pub fn sum_cyclic(
            &self,
            mut top: usize,
            mut left: usize,
            height: usize,
            width: usize,
        ) -> T {
            top %= self.H;
            left %= self.W;

            // 繰り返し回数
            let hrep: T = (height / self.H).try_into().unwrap();
            let wrep: T = (width / self.H).try_into().unwrap();

            // 右下の座標
            let bottom = (top + height) % self.H;
            let right = (left + width) % self.W;

            // 内部領域
            let S_inner = self.sum(.., ..) * hrep * wrep;

            // 左右の領域
            let S_lr = if left <= right {
                self.sum(.., left..right) * hrep
            } else {
                (self.sum(.., left..) + self.sum(.., ..right)) * hrep
            };

            // 上下の領域
            let S_tb = if top <= bottom {
                self.sum(top..bottom, ..) * wrep
            } else {
                (self.sum(top.., ..) + self.sum(..bottom, ..)) * wrep
            };

            // 端の領域
            let S_edge = match (top <= bottom, left <= right) {
                (true, true) => self.sum(top..bottom, left..right),
                (true, false) => self.sum(top..bottom, left..) + self.sum(top..bottom, ..right),
                (false, true) => self.sum(top.., left..right) + self.sum(..bottom, left..right),
                (false, false) => {
                    self.sum(top.., left..)
                        + self.sum(top.., ..right)
                        + self.sum(..bottom, left..)
                        + self.sum(..bottom, ..right)
                }
            };

            S_inner + S_lr + S_tb + S_edge
        }
    }
}

mod macro_chmax {
    #![allow(dead_code)]
    //! chmaxの実装
    /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmax {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmax! {
                $a,
                ($b).max($c)
                $(,$other)*
            }
        }}
    }
}

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
