#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut XY: [(f64, f64); N]
    }

    XY.push((0.0, 0.0));

    let mut cur = (0.0, 0.0);
    let mut ans = 0.0;

    for i in 0..N + 1 {
        let (a, b) = cur;
        let (c, d) = XY[i];

        let cost = ((a - c).powf(2.0) + (b - d).powf(2.0)).sqrt();
        debug!(cur, XY[i], cost);

        ans += cost;

        cur = XY[i];
    }

    println!("{ans}");
}
