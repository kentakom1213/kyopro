//             E - Count Median
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc169/tasks/abc169_e
// ----------------------------------------

#![allow(non_snake_case)]

use proconio::input;

/// ## 方針
/// 中央値の最小値、中央値の最大値の間の値を考える
/// Nが奇数のときは1刻み、Nが偶数のときは1/2刻みで
/// 中央値が現れる。
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
    }

    let (mut A, mut B) = (vec![], vec![]);
    for &(a, b) in &AB {
        A.push(a);
        B.push(b);
    }

    A.sort();
    B.sort();

    // A, Bそれぞれの中央値を考える
    if N % 2 == 0 {
        let med_a = (A[N / 2 - 1] + A[N / 2]);
        let med_b = (B[N / 2 - 1] + B[N / 2]);
        let ans = med_b - med_a + 1;
        println!("{}", ans);
    } else {
        let med_a = A[N / 2];
        let med_b = B[N / 2];
        let ans = med_b - med_a + 1;
        println!("{}", ans);
    }
}
