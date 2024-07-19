#![allow(non_snake_case)]

use cp_library_rs::{
    get,
    graph::rerooting::{examples::Diameter, Rerooting},
};

fn main() {
    let N = get!(usize);
    let edges = get!(usize, usize, isize; N - 1);

    let mut tree: Rerooting<Diameter> = Rerooting::new(N);

    for &(u, v, w) in &edges {
        tree.add_edge2(u, v, w);
    }

    tree.build();

    let ans = tree.ans.iter().max().unwrap();

    println!("{}", ans);
}
