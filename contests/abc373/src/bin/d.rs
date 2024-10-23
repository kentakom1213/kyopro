#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::weighted_union_find::WeightedUnionFind,
};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        UVW: [(Usize1, Usize1, isize); M]
    }

    let mut uf = WeightedUnionFind::<Add<isize>>::new(N);

    for &(u, v, w) in &UVW {
        if uf.unite(u, v, w).is_err() {
            unreachable!()
        }
    }

    println!("{}", (0..N).map(|i| uf.weight(i)).join(" "));
}
