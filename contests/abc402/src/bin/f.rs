#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, debug2D, number_theory::powmod::powmod};
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [[usize; N]; N]
    }

    debug2D!(A);

    // 半分全列挙
    // (0, 0) から (i, N-i-1) に到達するパスの値を列挙
    let lpath: Vec<Vec<usize>> = {
        let mut vals = vec![vec![]; N];
        for P in 0..1_usize << N - 1 {
            let mut r = 0;
            let mut c = 0;
            let mut val = A[r][c];
            for i in 0..N - 1 {
                if P >> i & 1 == 0 {
                    c += 1;
                } else {
                    r += 1;
                }
                val = val * 10 + A[r][c];
                val %= M;
            }
            vals[r].push(val);
        }
        vals
    };

    // 対角要素を 0 に
    for i in 0..N {
        A[i][N - i - 1] = 0;
    }

    // (i, N-i-1) から (N-1, N-1) に到達するパスの値を列挙
    let rpath: Vec<Vec<usize>> = {
        let mut vals = vec![vec![]; N];
        for P in 0..1_usize << N - 1 {
            let mut r = N - 1;
            let mut c = N - 1;
            let mut d = 1;
            let mut val = A[r][c];
            for i in 0..N - 1 {
                if P >> i & 1 == 0 {
                    c -= 1;
                } else {
                    r -= 1;
                }
                d *= 10;
                val += A[r][c] * d;
                val %= M;
            }
            vals[r].push(val);
        }
        for v in &mut vals {
            v.sort();
        }
        vals
    };

    debug2D!(lpath);
    debug2D!(rpath);

    // マージ
    let mut ans = 0;
    let offset = powmod(10, N - 1, M);

    for row in 0..N {
        for &lval in &lpath[row] {
            let lval = lval * offset % M;
            // (lval + rval) % M を最大化
            let mid = rpath[row].partition_point(|&v| v < M - lval);
            debug!(lval, mid, &rpath[row][..mid], &rpath[row][mid..]);

            if let Some(&last) = rpath[row][..mid].last() {
                chmax!(ans, (lval + last) % M);
            }
            if let Some(&last) = rpath[row][mid..].last() {
                chmax!(ans, (lval + last) % M);
            }
        }
    }

    println!("{ans}");
}
