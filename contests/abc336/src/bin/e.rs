// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
    }

    const T7: usize = 10_000_000;

    // 桁和を保存
    let DS = (0..=T7).map(|i| digit_sum(i)).collect_vec();

    // 上位7桁と下位7桁を分けて考える
    let top = N / T7;
    let bot = N % T7;

    debug!(top, bot);

    let DST = digit_sum(top);
    let TOP = top * T7;

    // 剰余の場合の数を保存
    let MOD = 9 * 7 + 1;
    let MOD2 = MOD * 2;

    // CNT[i][j][k] := 0~10000000 までの桁和がiで，jで割ったあまりがkになるもの
    let mut CNT = vec![vec![vec![0_usize; MOD2]; MOD2]; MOD];

    for n in 0..T7 {
        let i = digit_sum(n);
        for j in 1..MOD2 {
            let k = n % j;
            CNT[i][j][k] += 1;
        }
    }

    debug!(CNT);

    // 数え上げ
    let mut ans = 0;

    // topを固定 → bot以下である必要がある．
    for b in 0..=bot {
        if TOP + b > 0 && (TOP + b) % (DST + DS[b]) == 0 {
            ans += 1;
        }
    }

    // 上位部分がtop未満の場合
    for t in 0..top {
        // 上位部分
        let T = t * T7;
        // top部分の桁和
        let td = digit_sum(t);

        for bd in 0..=MOD {
            if td + bd == 0 {
                continue;
            }
            
            // 全体の桁和
            let ds = td + bd;
            // top部分 % 全体の桁和
            let mod_t = T % ds;
            // 求める剰余
            let req = ds - mod_t;

            ans += CNT[bd][ds][req];
        }
    }

    println!("{ans}");
}

fn digit_sum(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }
    ans
}
