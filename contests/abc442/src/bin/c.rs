#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, number_theory::comb_no_mod::comb};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g[b].push(a);
        g
    });

    debug2D!(G);

    for i in 0..N {
        let siz = N - G[i].len() - 1;
        let res = comb(siz, 3);
        debug!(G[i].len(), siz, res);

        print!("{res} ");
    }
    println!()
}
