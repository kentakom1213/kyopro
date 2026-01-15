#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add,
    data_structure::segment_tree::SegmentTree,
    debug,
    number_theory::modint::{Modint, M998},
};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        K: usize,
        P: [Usize1; N]
    }

    // 数列全体の転倒数
    let inv_all = {
        let mut seg = SegmentTree::<Add<M998>>::new(N);
        let mut res = M998::new(0);
        for &p in &P {
            res += seg.get_range(p..);
            *seg.get_mut(p).unwrap() += 1;
        }
        res
    };

    // 長さKの区間の転倒数の期待値
    let exp_k = M998::new(K) * (K - 1) / 2 / 2;

    // 長さKの区間全部について考える
    let mut ans = M998::new(0);
    let mut seg = SegmentTree::<Add<M998>>::new(N);
    let mut inv = M998::new(0);

    // 0..K で初期化
    for &p in &P[..K] {
        inv += seg.get_range(p..);
        *seg.get_mut(p).unwrap() += 1;
    }

    debug!(seg);
    debug!(inv);

    ans += inv_all - inv + exp_k;

    // ずらす
    for i in 0..N - K {
        let l = P[i];
        *seg.get_mut(l).unwrap() -= 1;
        inv -= seg.get_range(..l);

        let r = P[i + K];
        inv += seg.get_range(r..);
        *seg.get_mut(r).unwrap() += 1;

        debug!(seg);
        debug!(inv, l, r);

        ans += inv_all - inv + exp_k;
    }

    ans /= N - K + 1;

    println!("{ans}");
}
