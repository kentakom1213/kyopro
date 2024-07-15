#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use cp_library_rs::debug;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        T: String,
        LR: [(Usize1, usize); Q]
    }

    let C: Vec<usize> = T
        .chars()
        .map(|c| {
            let x = dec(c);
            x * x * x
        })
        .collect();

    debug!(C);

    // 3乗の累積和
    let mut S = vec![0; N + 1];
    for i in 0..N {
        S[i + 1] = S[i] + C[i];
    }

    debug!(S);

    let mut seen = HashSet::new();
    let mut memo = HashMap::new();

    for &(l, r) in &LR {
        let s = S[r] - S[l];

        if &T[l..r] == "153" {
            println!("0");
            continue;
        }

        if s == 153 {
            println!("1");
            continue;
        }

        let res = calc(s, &mut seen, &mut memo);

        if res == usize::MAX {
            println!("-1");
        } else {
            println!("{}", res + 1);
        }
    }
}

fn dec(x: char) -> usize {
    x as usize - '0' as usize
}

fn calc(x: usize, seen: &mut HashSet<usize>, memo: &mut HashMap<usize, usize>) -> usize {
    debug!(x, seen, memo);

    if x == 153 {
        return 0;
    }

    if let Some(&res) = memo.get(&x) {
        return res;
    }

    if seen.contains(&x) {
        return usize::MAX;
    }

    let sum = x
        .to_string()
        .chars()
        .map(|c| {
            let x = dec(c);
            x * x * x
        })
        .sum::<usize>();

    seen.insert(x);

    let res = calc(sum, seen, memo).saturating_add(1);

    memo.insert(x, res);

    res
}
