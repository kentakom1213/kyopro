#![allow(non_snake_case)]

use proconio::{fastout, input};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
        BC: [(usize, usize); Q]
    }

    let mut cnt = A.iter().fold(FxHashMap::default(), |mut map, &a| {
        *map.entry(a).or_insert(0) += 1;
        map
    });
    let mut sum = A.iter().sum::<usize>();

    for &(b, c) in &BC {
        if let Some(&k) = cnt.get(&b) {
            sum -= k * b;
            sum += k * c;
            cnt.remove(&b);
            *cnt.entry(c).or_default() += k;
        }

        println!("{sum}");
    }
}
