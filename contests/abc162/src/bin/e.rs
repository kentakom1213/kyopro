use cp_library_rs::{
    cfor, debug,
    number_theory::modint::{Fp, M107},
    utils::num_traits::Zero,
};
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        K: usize
    }

    let mut cnt = vec![M107::zero(); K + 1];

    for x in (1..=K).rev() {
        cnt[x] = M107::new(K / x).pow(N);

        cfor! {let mut i = 2; x * i <= K; i += 1;; {
            let tmp = cnt[x * i];
            cnt[x] -= tmp;
        }}
    }

    debug!(cnt);

    let ans = cnt
        .into_iter()
        .enumerate()
        .map(|(i, x)| x * i)
        .sum::<M107>();

    println!("{ans}");
}
