//                A - Make M
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc161/tasks/arc161_a
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
        let mut pivot_idx = 0;
        for (i, val) in arr.iter().enumerate() {
            if val == pivot {
                pivot_idx = i;
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
        Some(l)
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
                let med = &arr[begin .. end]
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

use intro_select::*;

// main
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }

    if N == 1 {
        println!("Yes");
        return;
    }

    let x = *select(&mut A, N / 2 + 1);
    
    let cnt = A.iter().filter(|v| v == &&x).count();

    debug!(x, cnt);

    if cnt > N / 2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
