#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add,
    data_structure::bit::BIT,
    debug,
    number_theory::modint::{Fp, M998},
};
use num::Zero;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        P: [Usize1; N],
    }

    // p[i] := i より左にある P[i] 未満の要素の数
    let p = {
        let mut bit = BIT::<Add<usize>>::new(N);
        let mut v = vec![0_usize; N];
        for (i, &p) in P.iter().enumerate() {
            v[i] = bit.prefix_sum(p);
            bit.add(p, 1);
        }
        v
    };

    // q[i] := i より右にある P[i] 未満要素の数
    let q = {
        let mut bit = BIT::<Add<usize>>::new(N);
        let mut v = vec![0_usize; N];
        for (i, &p) in P.iter().enumerate().rev() {
            v[i] = bit.prefix_sum(p);
            bit.add(p, 1);
        }
        v
    };

    debug!(p, q);

    // O(N^2) 時間の数え上げ
    let mut ans = M998::zero();
    let mut sum = M998::zero();

    for i in 0..N {
        // 2 = k - 1
        ans += p[i] * q[i];

        // 2 < k - 1
        if i >= 1 {
            ans += sum * M998::new(2).pow(i - 1) * q[i];
        }

        sum += M998::new(2).pow(i).inv() * p[i];
    }

    println!("{ans}");
}
