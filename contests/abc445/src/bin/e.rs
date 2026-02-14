#![allow(non_snake_case)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

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

        let factors: Vec<F> = A
            .iter()
            .map(|&a| {
                table
                    .factorize_pairs(a)
                    .into_iter()
                    .map(|(p, n)| (p, [n, 0]))
                    .collect()
            })
            .collect();
        let lcm = lcm_all(&factors).into_iter().collect::<HashMap<_, _>>();
        let mut mlcm = M998::one();
        for (&p, &[n, _]) in &lcm {
            mlcm *= M998::new(p).pow(n);
        }

        debug!(lcm, mlcm);

        for i in 0..N {
            let mut ans = mlcm;
            for &(p, [n, _]) in &factors[i] {
                let [fst, snd] = lcm[&p];
                if n == fst {
                    ans /= M998::new(p).pow(fst - snd);
                }
            }
            print!("{}{}", ans, (i == N - 1).endl());
        }
    }
}

fn lcm_all(arr: &[F]) -> F {
    let mut q: BinaryHeap<(Reverse<_>, F)> =
        BinaryHeap::from_iter(arr.iter().map(|v| (Reverse(v.len()), v.clone())));

    while q.len() > 1 {
        let (_, x) = q.pop().unwrap();
        let (_, y) = q.pop().unwrap();

        let new = lcm(&x, &y);

        q.push((Reverse(new.len()), new));
    }

    q.pop().unwrap().1
}

fn lcm(left: &F, right: &F) -> F {
    let mut res = vec![];
    for m in left.iter().merge_join_by(right, |l, r| l.0.cmp(&r.0)) {
        match m {
            itertools::EitherOrBoth::Left(&(p, n)) => {
                res.push((p, n));
            }
            itertools::EitherOrBoth::Right(&(p, n)) => {
                res.push((p, n));
            }
            itertools::EitherOrBoth::Both(&(lp, ln), &(rp, rn)) => {
                let mut new = [ln[0], ln[1], rn[0], rn[1]];
                new.sort_unstable();
                res.push((lp, [new[3], new[2]]));
            }
        }
    }
    res
}
