#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        mut A: [isize; N]
    }
    A.sort_by_key(|&v| Reverse(v));

    let init = {
        let mut v = vec![0; N];
        v[0] = K;
        v
    };
    let mut pq = BinaryHeap::from([(A[0] * K as isize, init.clone())]);
    let mut seen = FxHashSet::default();
    seen.insert(init);

    for _ in 0..X {
        let (sum, choice) = pq.pop().unwrap();

        println!("{sum}");

        // 妥協の仕方を列挙
        for i in 0..N - 1 {
            if choice[i] == 0 {
                continue;
            }
            let mut nchoice = choice.clone();
            nchoice[i] -= 1;
            nchoice[i + 1] += 1;
            if seen.contains(&nchoice) {
                continue;
            }
            let nval = sum + A[i + 1] - A[i];
            pq.push((nval, nchoice.clone()));
            seen.insert(nchoice);
        }
    }
}
