#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    number_theory::{comb::Comb, modint::M107},
    utils::num_traits::Zero,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let cmb = Comb::<M107>::new(202020);

    for k in 1..=N {
        let mut ans = M107::zero();

        for i in 1..=N {
            // i個のボールを選択する
            if i + (i - 1) * (k - 1) > N {
                // 選べない場合
                break;
            }

            // 選択できるスペース
            let n = N - (i - 1) * (k - 1);
            let tmp = cmb.comb(n, i);

            debug!(k, i, n, tmp);

            ans += tmp;
        }

        println!("{ans}");
    }
}
