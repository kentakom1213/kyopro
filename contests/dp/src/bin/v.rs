#![allow(non_snake_case)]

use cp_library_rs::{
    consts::INF,
    debug,
    rerooting::{Rerooting, TreeMonoid},
};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

static mut MOD: usize = 0;

fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); N - 1]
    }

    unsafe {
        MOD = M;
    }

    let mut tree: Rerooting<Colorize> = Rerooting::new(N);

    for &(u, v) in &edges {
        tree.add_edge2(u, v, ());
    }

    tree.build();

    println!("{}", tree.ans.iter().join("\n"));
}

struct Colorize;

impl TreeMonoid for Colorize {
    type T = usize;
    type W = ();
    fn id() -> Self::T {
        1
    }
    fn merge(x: &Self::T, y: &Self::T) -> Self::T {
        unsafe { x * y % MOD }
    }
    fn put_edge(x: &Self::T, _weight: &Self::W) -> Self::T {
        unsafe { (x + 1) % MOD }
    }
}
