#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use sparse_table::{Alg::Min, SparseTable};

use crate::get_ as get;

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize ;;);
    let queries = get!(usize, usize; Q);

    let tb = SparseTable::<Min>::build(&A);

    for &(l, r) in &queries {
        println!("{}", tb.get_range(l..r));
    }
}

mod get_macro {
    //! 入力用マクロ
    // [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    #[macro_export]
    macro_rules! get_ {
        ($t:ty) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.trim().parse::<$t>().unwrap()
            }
        };
        ($($t:ty),*) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let mut iter = line.split_whitespace();
                (
                    $(iter.next().unwrap().parse::<$t>().unwrap(),)*
                )
            }
        };
        ($t:ty ; $n:expr) => {
            (0..$n).map(|_|
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
            ).collect::<Vec<_>>()
        };
        ($t:ty ;;) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.split_whitespace()
                    .map(|t| t.parse::<$t>().unwrap())
                    .collect::<Vec<_>>()
            }
        };
        ($t:ty ;; $n:expr) => {
            (0..$n).map(|_|
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
    }
}

mod sparse_table {
    //! SparseTable
    use std::fmt;
    use std::ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    };
    pub trait Semilattice {
        /// 元の型
        type Val: fmt::Debug + Clone;
        /// 単位元
        const E: Self::Val;
        /// 可換な二項演算
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val;
    }
    #[derive(Debug)]
    pub struct SparseTable<S: Semilattice> {
        pub size: usize,
        table: Vec<Vec<S::Val>>,
        logs: Vec<usize>,
    }
    impl<S: Semilattice> SparseTable<S> {
        #[inline]
        fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
            let start = match range.start_bound() {
                Unbounded => 0,
                Excluded(&v) => v + 1,
                Included(&v) => v,
            };
            let end = match range.end_bound() {
                Unbounded => self.size,
                Excluded(&v) => v,
                Included(&v) => v + 1,
            };
            if start <= end && end <= self.size {
                Some((start, end))
            } else {
                None
            }
        }
        /// SparseTableを構築する
        pub fn build(arr: &[S::Val]) -> Self {
            let size = arr.len();
            // 区間取得用の配列
            let mut logs = vec![0; size + 1];
            for i in 2..=size {
                logs[i] = logs[i >> 1] + 1;
            }
            // テーブルの高さ
            let lg = logs[size] + 1;
            // 繰り返し適用した結果
            let mut table = vec![vec![]; lg];
            for i in 0..size {
                table[0].push(arr[i].clone());
            }
            for i in 1..lg {
                let mut j = 0;
                while j + (1 << i) <= size {
                    let a = &table[i - 1][j];
                    let b = &table[i - 1][j + (1 << (i - 1))];
                    let res = S::op(a, b);
                    table[i].push(res);
                    j += 1;
                }
            }
            Self { size, table, logs }
        }
        /// 区間取得
        pub fn get_range<R: RangeBounds<usize> + fmt::Debug>(&self, range: R) -> S::Val {
            let Some((start, end)) = self.parse_range(&range) else {
                panic!("The given range is wrong: {:?}", range);
            };
            if start >= end {
                return S::E;
            }
            let lg = self.logs[end - start];
            let left = &self.table[lg][start];
            let right = &self.table[lg][end - (1 << lg)];
            S::op(left, right)
        }
    }
    pub mod Alg {
        use super::Semilattice;
        /// 区間最小値
        #[derive(Debug)]
        pub struct Min;
        impl Semilattice for Min {
            type Val = usize;
            const E: Self::Val = usize::MAX;
            fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
                *x.min(y)
            }
        }
        /// 区間最大値
        #[derive(Debug)]
        pub struct Max;
        impl Semilattice for Max {
            type Val = isize;
            const E: Self::Val = isize::MAX;
            fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
                *x.max(y)
            }
        }
        /// 最大公約数
        pub fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        /// 区間最大公約数
        #[derive(Debug)]
        pub struct GCD;
        impl Semilattice for GCD {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
                gcd(*x, *y)
            }
        }
    }
}
