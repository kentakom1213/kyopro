#![allow(non_snake_case)]

use cp_library_rs::{debug, get, rerooting::Rerooting};

fn main() {
    let N = get!(usize);
    let AB = get!(usize, usize, isize; N - 1);

    let mut dp: Rerooting<isize, _, _, _> = Rerooting::new(N, || 0, |a, b| *a.max(b), |x, w| x + w);

    for &(u, v, w) in &AB {
        dp.add_edge2(u, v, w);
    }

    // 集約
    dp.build();

    debug!(dp.ans);

    for &x in &dp.ans {
        println!("{}", x);
    }
}
