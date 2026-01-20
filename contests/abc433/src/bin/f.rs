#![allow(non_snake_case)]
use cp_library_rs::{
    debug,
    number_theory::{comb::Comb, modint::M998},
};
use num::Zero;
use proconio::input;

fn main() {
    input! {
        S: String
    }
    let N = S.len();
    let A: Vec<_> = S.chars().map(|c| (c as u8 - '0' as u8) as usize).collect();

    let cnt = {
        let mut cnt = vec![[0; 11]; N + 1];
        for (i, &a) in A.iter().enumerate() {
            cnt[i + 1] = cnt[i];
            cnt[i + 1][a] += 1;
        }
        cnt
    };

    let cmb = Comb::<M998>::new(2001001);
    let mut ans = M998::zero();

    for i in 0..N {
        let a = A[i];
        let p = cnt[i][a];
        let q = cnt[N][a + 1] - cnt[i][a + 1];

        debug!(a, p, q);

        // for k in 1..=(p + 1).min(q) {
        //     let c1 = cmb.comb(p, k - 1);
        //     let c2 = cmb.comb(q, k);
        //     debug!(k, c1, c2);

        //     ans += c1 * c2;
        // }

        // ヴァンデルモンドの畳み込み
        // Σ_(k+l=n) binom(p, k) binom(q, l) = binom(p + q, n)

        ans += cmb.comb(p + q, p + 1);
    }

    println!("{ans}");
}
