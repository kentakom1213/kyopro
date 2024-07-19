#![allow(non_snake_case)]

use cp_library_rs::{get, graph::scc::SCC};

fn main() {
    let (N, M) = get!(usize, usize);
    let edges = get!(usize, usize; M);
    let Q = get!(usize);

    let mut scc = SCC::new(N);

    for &(u, v) in &edges {
        scc.add_edge(u, v);
    }

    scc.decompose();

    for _ in 0..Q {
        let (u, v) = get!(usize, usize);

        println!("{}", (scc.belongs_to[u] == scc.belongs_to[v]) as usize);
    }
}
