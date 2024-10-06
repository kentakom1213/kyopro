#![allow(non_snake_case)]

use cp_library_rs::utils::consts::IINF;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: isize,
        A: [isize; N]
    }

    // 二分探索
    let mut ok = -1;
    let mut ng = 2 * IINF;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let sum = A.iter().map(|&a| a.min(mid)).sum::<isize>();

        if sum <= M {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    if ok >= IINF {
        println!("infinite");
    } else {
        println!("{ok}");
    }
}

