use cp_library_rs::{
    debug, debug2D,
    number_theory::{comb::Comb, modint::M998},
    utils::num_traits::Zero,
};
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
    }

    let cmb = Comb::<M998>::new(2020202);
    let mut ans = M998::zero();

    let f = |h: usize, w: usize| cmb.comb(h * w, K);

    for h in 1..=H {
        for w in 1..=W {
            // 最小長方形がhxwになるような場合の数
            let mut cnt = f(h, w) - (f(h.saturating_sub(1), w) * 2 + f(h, w.saturating_sub(1)) * 2)
                + (f(h.saturating_sub(1), w.saturating_sub(1)) * 4
                    + f(h.saturating_sub(2), w)
                    + f(h, w.saturating_sub(2)))
                - (f(h.saturating_sub(2), w.saturating_sub(1)) * 2
                    + f(h.saturating_sub(1), w.saturating_sub(2)) * 2)
                + f(h.saturating_sub(2), w.saturating_sub(2));

            cnt *= (H - h + 1) * (W - w + 1);

            ans += cnt * h * w;
        }
    }

    ans /= f(H, W);

    println!("{ans}");
}
