#![allow(non_snake_case)]

use cp_library_rs::{data_structure::union_find::UnionFind, debug};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
    }

    let mut uf = UnionFind::new(N);

    for (i, &a) in A.iter().enumerate() {
        uf.unite(i, a);
    }

    let mut ans = vec![0; N];

    for (_, g) in uf.enum_groups() {
        let m = *g.iter().max().unwrap();
        for i in g {
            ans[i] = m;
        }
    }

    debug!(ans);

    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}
