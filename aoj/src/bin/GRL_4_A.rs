#![allow(non_snake_case)]

use cp_library_rs::{get, scc::SCC};

fn main() {
    let (N, M) = get!(usize, usize);
    let edges = get!(usize, usize; M);

    let mut scc = SCC::new(N);

    for &(u, v) in &edges {
        scc.add_edge(u, v);
    }

    scc.decompose();

    println!("{}", (scc.group_count != N) as usize);
}
