#![allow(non_snake_case)]

use std::fs;

use cp_library_rs::{
    debug,
    number_theory::{
        factorize_query::FactorTable,
        modint::{Fp, M998},
    },
    utils::boolutil::BoolUtil,
};
use num::One;
use proconio::{fastout, input};
use rustc_hash::FxHashMap;

type F = Vec<(usize, [usize; 2])>;

#[fastout]
fn main() {
    input! {
        T: usize,
    }

    let table = FactorTable::new(10_001_001);

    for _ in 0..T {
        input! {
            N: usize,
            A: [usize; N],
        }

        let facs: Vec<_> = A.iter().map(|&a| table.factorize_pairs(a)).collect();
        let mut lcm = FxHashMap::<usize, [usize; 2]>::default();

        for i in 0..N {
            for &(p, n) in &facs[i] {
                if let Some(mx) = lcm.get_mut(&p) {
                    if n > mx[0] {
                        mx[1] = mx[0];
                        mx[0] = n;
                    } else if n > mx[1] {
                        mx[1] = n;
                    }
                } else {
                    lcm.insert(p, [n, 0]);
                }
            }
        }

        let mlcm: M998 = lcm
            .iter()
            .map(|(&p, &[n, _])| M998::new(p).pow(n))
            .product();

        debug!(lcm);

        for i in 0..N {
            let mut ans = mlcm;

            for &(p, n) in &facs[i] {
                let [fst, snd] = lcm[&p];
                if n == fst {
                    ans /= M998::new(p).pow(fst - snd);
                }
            }

            print!("{}{}", ans, (i + 1 == N).endl());
        }
    }
}
