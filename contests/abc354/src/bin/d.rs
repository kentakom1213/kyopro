#![allow(non_snake_case)]

use proconio::input;

use crate::acc2d_cyclic::Acc2D;

fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    }

    let acc: Acc2D<usize> = Acc2D::new(&vec![vec![2, 1, 0, 1], vec![1, 2, 1, 0]]);

    let ans = acc.sum_cyclic(
        B.rem_euclid(2) as usize,
        A.rem_euclid(4) as usize,
        (D - B) as usize,
        (C - A) as usize,
    );

    println!("{ans}");
}

mod acc2d_cyclic {
    #![allow(dead_code)]
    //! トーラス上での区間和取得ができる2次元累積和
    use num_traits::Num;
    use std::{
        convert::{TryFrom, TryInto},
        fmt::Debug,
        ops::{
            Bound::{Excluded, Included, Unbounded},
            Mul, RangeBounds,
        },
    };
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
            let wrep: T = (width / self.W).try_into().unwrap();
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
