#![allow(non_snake_case)]

use cp_library_rs::{
    debug, get,
    graph::rerooting::{examples::Diameter, Rerooting},
};

fn main() {
    let N = get!(usize);
    let AB = get!(usize, usize, isize; N - 1);

    let mut dp: Rerooting<Diameter> = Rerooting::new(N);

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
