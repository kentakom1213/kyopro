// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::gcd;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use sparse_table::{Alg::GCD, SparseTable};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        B: [usize; N],
        queries: [(Usize1, usize, Usize1, usize); Q]
    }

    // 隣接項の差分の絶対値
    let Da = (1..N).map(|i| A[i - 1].abs_diff(A[i])).collect_vec();
    let Db = (1..N).map(|i| B[i - 1].abs_diff(B[i])).collect_vec();

    let tableA = SparseTable::<GCD>::build(&Da);
    let tableB = SparseTable::<GCD>::build(&Db);

    for &(t, b, l, r) in &queries {
        let res = [
            A[t] + B[l],
            tableA.get_range(t..b - 1),
            tableB.get_range(l..r - 1),
        ]
        .into_iter()
        .fold(0, gcd);

        println!("{res}");
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
            type Val = isize;
            const E: Self::Val = isize::MAX;
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
