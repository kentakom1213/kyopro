#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M],
        Q: usize,
        CD: [(usize, usize); Q],
    }

    let mut items = AB
        .iter()
        .flat_map(|&(l, r)| [Item::IL(l), Item::IR(r)])
        .chain(
            CD.iter()
                .enumerate()
                .flat_map(|(q, &(c, d))| [Item::QL { i: c, q }, Item::QR { i: d, q }]),
        )
        .sorted()
        .collect_vec();

    debug!(items);

    // Mo's Algorithm
    

    println!("{}", ans.iter().join("\n"));
}

#[derive(Debug, PartialEq, Eq)]
enum Item {
    IL(usize),
    IR(usize),
    QL { i: usize, q: usize },
    QR { i: usize, q: usize },
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = match self {
            Item::IL(x) => x,
            Item::IR(x) => x,
            Item::QL { i, .. } => i,
            Item::QR { i, .. } => i,
        };
        let b = match other {
            Item::IL(x) => x,
            Item::IR(x) => x,
            Item::QL { i, .. } => i,
            Item::QR { i, .. } => i,
        };
        a.partial_cmp(b)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
