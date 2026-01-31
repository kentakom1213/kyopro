#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{actedmonoid::ActedMonoid, operation::Max},
    data_structure::{lazy_segment_tree::LazySegmentTree, segment_tree::SegmentTree},
};
use proconio::{fastout, input};

use crate::monoids::AddArithSum;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64; N]
    }

    let mut seg = SegmentTree::<Max<i64>>::from_vec(A.clone());

    let mut ans = LazySegmentTree::<AddArithSum<i64>>::new(N + 1);
}

mod monoids {
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, Sub};

    use num::{FromPrimitive, Zero};

    use super::ActedMonoid;

    /// 等差数列加算（一次式加算）+ 区間和
    ///
    /// Act = (p, q): すべての i に対し a[i] += p*i + q
    ///
    /// Val = (sum, cnt, sum_i)
    ///   sum   = 区間の a の総和
    ///   cnt   = 区間長
    ///   sum_i = 区間内の添字 i の総和
    #[derive(Debug)]
    pub struct AddArithSum<T>(PhantomData<T>);

    impl<T> ActedMonoid for AddArithSum<T>
    where
        T: Zero
            + Clone
            + PartialEq
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + FromPrimitive,
    {
        type Val = (T, usize, T);
        type Act = (T, T); // (p, q)

        fn e() -> Self::Val {
            (T::zero(), 0, T::zero())
        }
        fn id() -> Self::Act {
            (T::zero(), T::zero())
        }

        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            let (sx, cx, ix) = x.clone();
            let (sy, cy, iy) = y.clone();
            (sx + sy, cx + cy, ix + iy)
        }

        fn mapping(x: &Self::Val, f: &Self::Act) -> Self::Val {
            let (sum, cnt, sum_i) = x.clone();
            let (p, q) = f.clone();
            let cnt_t = T::from_usize(cnt).unwrap();
            // sum += p * (sum_i) + q * cnt
            (sum + p * sum_i + q * cnt_t, cnt, sum_i)
        }

        fn compose(old: &Self::Act, add: &Self::Act) -> Self::Act {
            // apply_lazy で lazy[k] = compose(lazy[k], f) なので
            // 「既存 old のあとに新しい add を足す」合成になっている必要がある
            // 今回は加算なので単純に足し算でよい
            let (p1, q1) = old.clone();
            let (p2, q2) = add.clone();
            (p1 + p2, q1 + q2)
        }
    }
}
