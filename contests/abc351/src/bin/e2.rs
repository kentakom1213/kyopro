#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(i64, i64); N]
    }

    // パリティで分割
    let (even, odd): (Vec<_>, Vec<_>) = XY.iter().copied().partition(|&(x, y)| (x + y) % 2 == 0);

    let ans = calc(&even) + calc(&odd);

    println!("{ans}");
}

fn calc(p: &[(i64, i64)]) -> i64 {
    if p.len() == 0 {
        return 0;
    }

    // 45度回転
    let (mut X, mut Y): (Vec<_>, Vec<_>) = p.iter().map(|(x, y)| (x + y, x - y)).unzip();

    X.sort_unstable();
    Y.sort_unstable();

    debug!(X, Y);

    let mut ans = 0;
    let mut xsum = 0;
    let mut ysum = 0;
    let mut cnt = 0;

    for (x, y) in X.into_iter().zip(Y) {
        ans += x * cnt - xsum;
        ans += y * cnt - ysum;
        xsum += x;
        ysum += y;
        cnt += 1;
    }

    ans / 2
}
