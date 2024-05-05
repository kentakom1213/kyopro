#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use itertools::Itertools;
use num_integer::Roots;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N]
    }

    // K回以内の操作で，Aiをすべてxの倍数にすることは可能か判定
    let can = |x: isize| -> bool {
        // 各要素をxで割ったあまり
        let mut D = A
            .iter()
            .map(|&a| a % x)
            .filter(|&d| d > 0)
            .sorted()
            .collect_vec();

        let n = D.len();

        // 左右から消していく\
        let (mut l, mut r) = (0, n - 1);
        let mut cnt = 0;

        while l < r {
            if D[l] <= x - D[r] {
                D[r] += D[l];
                cnt += D[l];
                l += 1;
            } else {
                D[l] -= x - D[r];
                cnt += x - D[r];
                r -= 1;
            }
            debug!(l, r, D);
        }

        cnt <= K
    };

    // Aの総和の約数を全探索
    for x in factors(A.iter().sum()) {
        if x == 1 || can(x) {
            println!("{x}");
            return;
        }
    }
}

const INF: usize = 1001001001001001001;

/// 約数を列挙する（降順に返す）
fn factors(n: isize) -> Vec<isize> {
    let mut res = vec![];

    for i in 1..=n.sqrt() + 1 {
        if i * i == n {
            res.push(i);
        } else if n % i == 0 {
            res.push(i);
            res.push(n / i);
        }
    }

    res.sort();
    res.reverse();
    res
}
