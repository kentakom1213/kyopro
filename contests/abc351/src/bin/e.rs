#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        XY: [(isize, isize); N]
    }

    // パリティで分類
    let (XY0, XY1) = XY
        .iter()
        .fold((vec![], vec![]), |(mut v0, mut v1), &(x, y)| {
            if (x + y) % 2 == 0 {
                v0.push((x, y));
            } else {
                v1.push((x, y));
            }
            (v0, v1)
        });

    // それぞれについて，マンハッタン距離の総和を求める
    let mut ans = 0;

    for XY in [XY0, XY1] {
        // 45度回転
        let xy45 = XY.iter().map(|&(x, y)| (x + y, x - y)).collect_vec();

        debug!(XY);
        debug!(xy45);

        // x軸とy軸を独立に考えれば良い（2で割る）
        let (mut X, mut Y): (Vec<isize>, Vec<isize>) = xy45.iter().cloned().unzip();

        X.sort();
        Y.sort();

        debug!(X, Y);

        debug!(diff_sum(&X), diff_sum(&Y));

        // 差分の総和
        ans += diff_sum(&X) + diff_sum(&Y);
    }

    ans /= 2;

    println!("{ans}");
}

/// 差分の総和を取る(ソート済み)
fn diff_sum(a: &[isize]) -> isize {
    // 累積和
    let N = a.len();
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + a[i];
    }

    debug!(S);

    let mut ans = 0;

    for i in 1..N {
        ans += (S[N] - S[i]) - (N - i) as isize * a[i - 1];
    }

    ans
}
