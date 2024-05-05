//         C - Linear Approximation
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc102/tasks/arc100_a
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

pub mod intro_select {
    use itertools::Itertools;

    /// ## partition
    /// arrの要素をpivot以下とpivotより大きいものに分割
    pub fn partition<T: Ord + std::fmt::Debug>(arr: &mut [T], pivot: &T) -> Option<usize> {
        // pivotの位置
        let mut pivot_cnt = 0;
        let mut pivot_idx = 0;
        for (i, val) in arr.iter().enumerate() {
            if val == pivot {
                pivot_idx = i;
                pivot_cnt += 1;
            }
        }
        if &arr[pivot_idx] != pivot {
            return None;
        }
        // 一番右のインデックス
        let r = arr.len() - 1;
        // pivotを一番右側に移動
        arr.swap(pivot_idx, r);
        // 移動していく
        let mut l = 0;
        for i in 0..r {
            if &arr[i] <= pivot {
                arr.swap(l, i);
                l += 1;
            }
        }
        // 末尾と交換
        arr.swap(l, r);
        // 中央のピボットを選択
        let left = l + 1 - pivot_cnt;
        let right = l;
        let mid = r / 2;
        let new_p = if mid < left {
            left
        } else if mid <= right {
            mid
        } else {
            right
        };
        Some(new_p)
    }

    /// ## select
    /// 配列arrのk番目の値を取り出す：O(N)
    pub fn select<T: Ord + Clone + std::fmt::Debug>(arr: &mut [T], k: usize) -> &T {
        if arr.len() == 1 {
            return &arr[0];
        }

        // 5つずつの要素にわけ、中央値をとる
        let size = (arr.len() + 4) / 5;
        let mut medians = {
            let mut medians = vec![];

            for i in 0..size {
                let begin = 5 * i;
                let end = arr.len().min(begin + 5);
                let med = &arr[begin..end]
                    .iter()
                    .sorted()
                    .nth((end - begin - 1) / 2)
                    .unwrap();
                medians.push(med.clone().clone());
            }
            medians
        };

        // 中央値の中央値
        let pvt = select(&mut medians, (size - 1) / 2);

        // ピボットの位置
        let pvt_idx = partition(arr, pvt).unwrap();

        if pvt_idx < k {
            select(&mut arr[pvt_idx + 1..], k - pvt_idx - 1)
        } else if pvt_idx > k {
            select(&mut arr[..pvt_idx], k)
        } else {
            &arr[k]
        }
    }
}

use intro_select::select;

// main
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    let mut A = A
        .iter()
        .zip(0..N as isize)
        .map(|(&v, i)| v - i)
        .collect_vec();

    let med = *select(&mut A, (N - 1) / 2);

    let ans = A.iter().map(|&v| (v - med).abs()).sum::<isize>();

    println!("{}", ans);
}
