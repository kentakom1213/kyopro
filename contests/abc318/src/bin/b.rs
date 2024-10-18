#![allow(non_snake_case)]

use cp_library_rs::data_structure::acc2d::acc2D;
use proconio::input;

fn main() {
    input! {
        N: usize,
        ABCD: [(usize, usize, usize, usize); N]
    }

    let mut arr = vec![vec![0; 110]; 110];

    for &(t, b, l, r) in &ABCD {
        arr[l][t] += 1;
        arr[l][b] -= 1;
        arr[r][t] -= 1;
        arr[r][b] += 1;
    }

    let S = acc2D(&arr);

    let mut ans = 0;

    for r in 0..105 {
        for c in 0..105 {
            if S(0, r, 0, c) > 0 {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
