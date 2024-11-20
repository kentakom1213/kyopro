#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String
    }

    let mut ans = vec![0];

    let mut sum = 0;

    for (i, c) in S.chars().enumerate() {
        let x = c as usize - '0' as usize;
        sum += x * (i + 1);
        ans.push(sum);
    }

    debug!(ans);

    for i in (1..=N).rev() {
        ans[i - 1] += ans[i] / 10;
        ans[i] %= 10;
    }

    debug!(ans);

    if ans[0] == 0 {
        println!("{}", ans[1..].iter().join(""));
    } else {
        println!("{}", ans.iter().join(""));
    }
}
