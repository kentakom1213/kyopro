#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut map = vec![0; N + 1];

    for x in 1.. {
        for y in x + 1.. {
            let sum = x * x + y * y;
            if sum > N {
                break;
            }
            map[sum] += 1;
        }
        if x * x > N {
            break;
        }
    }

    let ans = map
        .into_iter()
        .enumerate()
        .filter(|&(_, n)| n == 1)
        .map(|(x, _)| x)
        .collect::<Vec<_>>();

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
