#![allow(non_snake_case)]

use cp_library_rs::{
    extmonoid::ExtMonoid,
    lazy_segment_tree::LazySegmentTree,
    modint::{Modint, M998},
};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [M998; N],
        B: [M998; N],
    }

    let mut seg = LazySegmentTree::<AddSumMod>::from(
        &A.iter()
            .zip(&B)
            .map(|(&a, &b)| (1, a, b, a * b))
            .collect_vec(),
    );

    for _ in 0..Q {
        input! {t: usize}

        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: M998,
                }
                seg.apply(l..r, (x.into(), 0.into()));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: M998,
                }
                seg.apply(l..r, (0.into(), x.into()));
            }
            3 => {
                input! {
                    l: Usize1,
                    r: usize
                }
                let res = seg.get(l..r).3;
                println!("{res}");
            }
            _ => (),
        }
    }
}

/// ## AddSum
/// - 区間加算
/// - 区間和
#[derive(Debug)]
pub struct AddSumMod;
impl ExtMonoid for AddSumMod {
    // (a, b, a*b)
    type X = (usize, M998, M998, M998);
    // (fa, fb)
    type F = (M998, M998);
    fn id_x() -> Self::X {
        (0, Modint(0), Modint(0), Modint(0))
    }
    fn id_f() -> Self::F {
        (Modint(0), Modint(0))
    }
    fn op(x: &Self::X, y: &Self::X) -> Self::X {
        let &(xn, xa, xb, xab) = x;
        let &(yn, ya, yb, yab) = y;
        (xn + yn, xa + ya, xb + yb, xab + yab)
    }
    fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
        let &(n, a, b, ab) = x;
        let &(fa, fb) = y;
        (
            n,
            a + fa * n,
            b + fb * n,
            ab + fb * a + fa * b + fa * fb * n,
        )
    }
    fn composition(x: &Self::F, y: &Self::F) -> Self::F {
        let &(xa, xb) = x;
        let &(ya, yb) = y;
        (xa + ya, xb + yb)
    }
    fn aggregate(x: &Self::F, p: usize) -> Self::F {
        // let &(fa, fb) = x;
        // (fa * p, fb * p)
        *x
    }
}
