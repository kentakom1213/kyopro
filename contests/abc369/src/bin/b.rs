#![allow(non_snake_case)]

use cp_library_rs::utils::consts::INF;
use proconio::input;

fn main() {
    input! {
        N: usize,
        AS: [(usize, char); N]
    }

    let mut l = INF;
    let mut r = INF;
    let mut ans = 0;

    for &(a, s) in &AS {
        if l == INF && s == 'L' {
            l = a;
        }
        if r == INF && s == 'R' {
            r = a;
        }
    }

    for &(a, s) in &AS {
        if s == 'L' {
            ans += l.abs_diff(a);
            l = a;
        } else {
            ans += r.abs_diff(a);
            r = a;
        }
    }

    println!("{ans}");
}
