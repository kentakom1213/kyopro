#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut ans = vec![A[0]];

    for i in 1..N {
        while *ans.last().unwrap() != A[i] {
            let x = *ans.last().unwrap();
            if x > A[i] {
                ans.push(x - 1);
            } else {
                ans.push(x + 1);
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}
