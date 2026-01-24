#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid_with_context::MonoidCtx,
    debug,
    graph::rerooting::{RerootingDP, TreeMonoid},
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        UV: [(Usize1, Usize1); N - 1]
    }

    let mut dp = RerootingDP::new(N, DP);

    for (i, &(u, v)) in UV.iter().enumerate() {
        dp.add_edge(u, v, i, i);
    }

    let ans = dp.build(0);

    debug!(ans);

    for a in ans {
        println!("{}", a.sum);
    }
}

#[derive(Debug, Clone)]
struct D {
    cnt: usize,
    sum: usize,
}
struct DP;

impl MonoidCtx for DP {
    type Val = D;
    fn e(&self) -> Self::Val {
        D { cnt: 0, sum: 0 }
    }
    fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
        D {
            cnt: left.cnt + right.cnt,
            sum: left.sum + right.sum,
        }
    }
}

impl TreeMonoid for DP {
    type T = D;
    fn put_edge(&self, x: &Self::T, _i: usize) -> Self::Val {
        D {
            cnt: x.cnt,
            // 頂点の数だけ増える
            sum: x.sum + x.cnt,
        }
    }
    fn put_vertex(&self, x: &Self::Val, _v: usize) -> Self::T {
        D {
            // 頂点を増やす
            cnt: x.cnt + 1,
            sum: x.sum,
        }
    }
}
