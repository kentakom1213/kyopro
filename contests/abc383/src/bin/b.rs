#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug};
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        H: usize,
        W: usize,
        D: usize,
        S: [Chars; H]
    }

    let mut ans = 0;

    for (ar, ac, br, bc) in iproduct!(0..H, 0..W, 0..H, 0..W) {
        if (ar, ac) == (br, bc) {
            continue;
        }
        if S[ar][ac] == '#' || S[br][bc] == '#' {
            continue;
        }

        debug!(ar, ac, br, bc);

        let mut cnt = 0;

        for (r, c) in iproduct!(0..H, 0..W) {
            if S[r][c] == '.'
                && (r.abs_diff(ar) + c.abs_diff(ac) <= D || r.abs_diff(br) + c.abs_diff(bc) <= D)
            {
                debug!(r, c);
                cnt += 1;
            }
        }

        chmax!(ans, cnt);
    }

    println!("{ans}");
}
