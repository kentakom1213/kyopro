#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        A: [Chars; N]
    }

    let mut B = vec![vec![' '; N]; N];

    let mut nth = vec![vec![]; N / 2];

    for i in 0..N {
        for j in 0..N {
            let n = i.min(j).min(N - i - 1).min(N - j - 1);
            nth[n].push((i, j));
        }
    }

    // 転置
    for n in 0..N / 2 {
        for &(i, j) in &nth[n] {
            B[i][j] = match n % 4 {
                0 => A[N - j - 1][i],
                1 => A[N - i - 1][N - j - 1],
                2 => A[j][N - i - 1],
                _ => A[i][j],
            }
        }
    }

    for i in 0..N {
        println!("{}", B[i].iter().join(""));
    }
}
