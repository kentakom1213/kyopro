#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, utils::consts::INF};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: Chars,
        C: [usize; N]
    }

    // L0[i] := i文字目を0，それ以前をしましまにするときのコスト
    let mut L0 = vec![0; N];
    // L1[i] := i文字目を1，それ以前をしましまにするときのコスト
    let mut L1 = vec![0; N];

    if S[0] == '0' {
        L1[0] = C[0];
    } else {
        L0[0] = C[0];
    }

    for i in 1..N {
        if S[i] == '0' {
            L0[i] = L1[i - 1];
            L1[i] = L0[i - 1] + C[i];
        } else {
            L0[i] = L1[i - 1] + C[i];
            L1[i] = L0[i - 1];
        }
    }

    debug!(L0, L1);

    // R0[i] := i文字目を0，それ以降をしましまにするときのコスト
    let mut R0 = vec![0; N];
    // R1[i] := i文字目を1，それ以降をしましまにするときのコスト
    let mut R1 = vec![0; N];

    if S[N - 1] == '0' {
        R1[N - 1] = C[N - 1];
    } else {
        R0[N - 1] = C[N - 1];
    }

    for i in (0..N - 1).rev() {
        if S[i] == '0' {
            R0[i] = R1[i + 1];
            R1[i] = R0[i + 1] + C[i];
        } else {
            R0[i] = R1[i + 1] + C[i];
            R1[i] = R0[i + 1];
        }
    }

    debug!(R0, R1);

    // マージ
    let mut ans = INF;

    for i in 0..N - 1 {
        chmin!(ans, L0[i] + R0[i + 1], L1[i] + R1[i + 1],);
    }

    println!("{ans}");
}
