#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        M: usize,
        K: usize
    }

    // 実験でわかる部分
    if M == 0 {
        if K == 0 {
            println!("0 0");
        } else {
            println!("-1");
        }
    } else if M == 1 {
        if K == 0 {
            println!("0 0 1 1");
        } else {
            println!("-1");
        }
    } else {
        // 構築が不可能
        if 1 << M <= K {
            println!("-1");
            return;
        }
        // K以外の数
        let rem = (0..1 << M).filter(|&i| i != K).collect_vec();

        let mut ans = vec![];

        for &i in &rem {
            ans.push(i);
        }

        ans.push(K);

        for &i in rem.iter().rev() {
            ans.push(i);
        }

        ans.push(K);

        println!("{}", ans.iter().join(" "));
    }
}
