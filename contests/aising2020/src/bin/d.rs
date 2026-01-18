#![allow(non_snake_case)]

use cp_library_rs::{debug, number_theory::powmod::powmod};
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        X: String
    }
    let X = X.chars().map(|c| (c == '1') as usize).collect_vec();
    let P = X.iter().sum::<usize>();

    if P == 0 {
        for _ in 0..N {
            println!("1");
        }
        return;
    }

    let Pdec = P - 1;
    let Pinc = P + 1;
    debug!(Pdec, Pinc);

    let Mdec = (Pdec > 0).then(|| X.iter().fold(0, |acc, d| (2 * acc + d) % Pdec));
    let Minc = X.iter().fold(0, |acc, d| (2 * acc + d) % Pinc);
    debug!(Mdec, Minc);

    for (i, &x) in X.iter().enumerate() {
        let d = N - i - 1;
        debug!(d, x);

        if x == 1 {
            // 1を減らす
            if let Some(Mdec) = Mdec {
                let delta = powmod(2, d, Pdec);
                let val = (Pdec + Mdec - delta) % Pdec;
                debug!(Mdec, delta, val);

                println!("{}", count(val) + 1);
            } else {
                println!("0");
            }
        } else {
            // 1を増やす
            let delta = powmod(2, d, Pinc);
            let val = (Minc + delta) % Pinc;
            debug!(Minc, delta, val);

            println!("{}", count(val) + 1);
        }
    }
}

fn count(mut x: usize) -> usize {
    let mut cnt = 0;
    while x != 0 {
        x %= x.count_ones() as usize;
        cnt += 1;
    }
    cnt
}
