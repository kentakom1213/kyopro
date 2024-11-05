#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, geometry::vec2::Vec2};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: f64,
        T: f64,
        ABCD: [[(f64, f64); 2]; N]
    }

    debug!(ABCD);

    let mut ans = 1e20;

    for perm in ABCD.iter().permutations(N) {
        for cmb in 0..1 << N {
            let mut cur = Vec2(0.0, 0.0);
            let mut total = 0.0;

            // debug!(perm, cmb);

            for i in 0..N {
                let flag = cmb >> i & 1;
                let start = perm[i][flag].into();
                let end = perm[i][flag ^ 1].into();

                let a = cur.dist(start) / S;
                let b = start.dist(end) / T;
                // debug!(a, b);

                total += a + b;
                cur = end;
            }

            // debug!(perm, cmb, total);

            chmin!(ans, total);
        }
    }

    println!("{ans}");
}
