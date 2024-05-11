#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N]
    }

    // シミュレーション
    let mut i = 0;
    let mut cnt = 1;
    let mut vacant = K;

    while i < N {
        if vacant >= A[i] {
            // 乗せる
            vacant -= A[i];
            i += 1;
        } else {
            // 発車させる
            cnt += 1;
            vacant = K;
        }
    }

    println!("{cnt}");
}
