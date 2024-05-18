#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut SC: [(String, usize); N]
    }

    // ソート
    SC.sort();

    let T = SC.iter().map(|&(_, t)| t).sum::<usize>();
    let idx = T % N;

    let ans = &SC[idx].0;

    println!("{ans}");
}

const _INF: usize = 1001001001001001001;
