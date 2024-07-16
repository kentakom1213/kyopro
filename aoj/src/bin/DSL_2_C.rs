#![allow(non_snake_case)]

use std::collections::{BTreeMap, BTreeSet};

use cp_library_rs::{debug, get};

fn main() {
    let N = get!(usize);
    let P = get!(isize, isize; N);
    let Q = get!(usize);

    let mut tree: BTreeMap<isize, BTreeMap<isize, usize>> = BTreeMap::new();

    for (i, &(x, y)) in P.iter().enumerate() {
        tree.entry(x).or_default().insert(y, i);
    }

    debug!(tree);

    for _ in 0..Q {
        let (sx, tx, sy, ty) = get!(isize, isize, isize, isize);

        let mut res = vec![];

        for (_, ys) in tree.range(sx..=tx) {
            for (_, &i) in ys.range(sy..=ty) {
                res.push(i);
            }
        }

        res.sort();

        for i in res {
            println!("{}", i);
        }

        println!();
    }
}
