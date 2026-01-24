#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid_with_context::MonoidCtx,
    debug,
    graph::rerooting::{RerootingDP, TreeMonoid},
};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        C: [usize; N]
    }

    let mut tree = RerootingDP::new(N, DP(C));

    for (i, &(a, b)) in AB.iter().enumerate() {
        tree.add_edge(a, b, i, i);
    }

    let res = tree.build(0);

    debug!(res);

    let ans = res.iter().map(|t| t.ans).min().unwrap();

    println!("{ans}");
}

#[derive(Debug, Clone)]
struct T {
    sum: usize,
    ans: usize,
}

struct DP(Vec<usize>);

impl MonoidCtx for DP {
    type Val = T;
    fn e(&self) -> Self::Val {
        T { sum: 0, ans: 0 }
    }
    fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
        T {
            sum: left.sum + right.sum,
            ans: left.ans + right.ans,
        }
    }
}

impl TreeMonoid for DP {
    type T = T;
    fn put_edge(&self, x: &Self::T, _i: usize) -> Self::Val {
        T {
            sum: x.sum,
            ans: x.ans + x.sum,
        }
    }
    fn put_vertex(&self, x: &Self::Val, v: usize) -> Self::T {
        T {
            sum: x.sum + self.0[v],
            ans: x.ans,
        }
    }
}
