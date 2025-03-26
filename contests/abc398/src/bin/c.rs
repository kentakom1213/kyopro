#![allow(non_snake_case)]

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let cnt = A
        .into_iter()
        .zip(1..)
        .fold(BTreeMap::new(), |mut map, (a, i)| {
            map.entry(a).or_insert(vec![]).push(i);
            map
        });

    let ans = cnt
        .into_iter()
        .rev()
        .find(|(_, idxs)| idxs.len() == 1)
        .map(|(_, idxs)| idxs[0])
        .unwrap_or(-1);

    println!("{}", ans);
}
