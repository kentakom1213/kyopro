#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        TX: [(usize, Usize1); N]
    }

    // 途中で入手できるアイテムの種類とその位置
    let mut items = vec![vec![]; N];

    for (i, &(t, x)) in TX.iter().enumerate() {
        if t == 1 {
            items[x].push(i);
        }
    }

    debug2D!(items);

    // 後ろから見る
    let mut cmd = vec![0; N];

    for (i, &(t, x)) in TX.iter().enumerate().rev() {
        if t == 2 {
            // 可能かどうか判定
            let mut isok = false;
            // 自分より前に手に入れられるアイテムを探す
            while let Some(idx) = items[x].pop() {
                if idx < i {
                    isok = true;
                    cmd[idx] = 1;
                    break;
                }
            }
            if !isok {
                println!("-1");
                return;
            }
            cmd[i] = -1;
        }
    }

    debug!(cmd);

    // 累積和の最大値
    let mut ans = 0;
    let mut sum = 0;
    for i in 0..N {
        sum += cmd[i];
        ans = ans.max(sum);
    }
    println!("{ans}");
    println!("{}", cmd.iter().filter(|&&i| i >= 0).join(" "));
}
