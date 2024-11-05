#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::{debug, utils::consts::IINF};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: [isize; N],
        L: [isize; N]
    }

    // k = k にしたとき，宝をとりきることができるか
    let can = |k: isize| -> bool {
        X.iter()
            .map(|&x| (x - k).abs())
            .sorted()
            .zip(&L)
            .all(|(x, &l)| x <= l)
    };

    // 境界の候補を全探索
    let mut bound = BTreeSet::default();

    bound.insert(-IINF);
    bound.insert(IINF);

    for &x in &X {
        for &l in &L {
            for d in -1..=1 {
                bound.insert(x + l + d);
                bound.insert(x - l + d);
            }
        }
    }

    // 加算
    let mut ans = 0;
    let mut first_ok = -IINF;
    let mut prv = false;

    for &x in &bound {
        let cur = can(x);

        debug!(x, cur);

        match (prv, cur) {
            (false, false) => (),
            (false, true) => first_ok = x,
            (true, false) => ans += x - first_ok,
            (true, true) => (),
        }

        prv = cur;
    }

    println!("{ans}");
}
