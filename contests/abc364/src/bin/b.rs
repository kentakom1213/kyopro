#![allow(non_snake_case)]

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        H: usize,
        W: usize,
        si: Usize1,
        sj: Usize1,
        C: [Chars; H],
        X: String
    }

    let mut ci = si;
    let mut cj = sj;

    for x in X.chars() {
        let (ni, nj) = match x {
            'L' => (ci, cj.saturating_sub(1)),
            'R' => (ci, cj + 1),
            'U' => (ci.saturating_sub(1), cj),
            'D' => (ci + 1, cj),
            _ => unreachable!(),
        };

        if ni < H && nj < W && C[ni][nj] == '.' {
            ci = ni;
            cj = nj;
        }
    }

    println!("{} {}", ci + 1, cj + 1);
}
