#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use cp_library_rs::{
    debug, get,
    graph::dijkstra::path_reconstruction,
    utils::{num_traits::Bounded, ord_float::OrdF64},
};

const MAX_V: usize = 3;

fn main() {
    while solve() {}
}

fn solve() -> bool {
    let (N, M) = get!(usize, usize);
    if N == 0 && M == 0 {
        return false;
    }
    let (S, T) = get!(usize1, usize1);
    let edges = get!(usize1, usize1, usize, usize; M);

    

    true
}
