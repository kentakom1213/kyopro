#![allow(non_snake_case)]

use cp_library_rs::utils::consts::IINF;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
        B: [isize; N - 1],
    }

    A.sort();

    let mut ok = IINF;
    let mut ng = 0;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let B_mid = B.iter().cloned().chain([mid]).sorted().collect_vec();

        let isok = A.iter().zip(&B_mid).all(|(a, b)| a <= b);

        if isok {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if ok == IINF {
        println!("-1");
    } else {
        println!("{ok}");
    }
}
