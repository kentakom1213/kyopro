#![allow(non_snake_case)]

use proconio::input;

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        T: usize,
        AB: [(usize, usize); M]
    }

    let mut t = 0;
    let mut cap = N;

    for &(a, b) in &AB {
        // 減らす
        cap = cap.saturating_sub(a - t);
        if cap == 0 {
            println!("No");
            return;
        }
        // 増やす
        cap = N.min(cap + b - a);
        t = b;
    }

    if T - t >= cap {
        println!("No");
    } else {
        println!("Yes");
    }
}
