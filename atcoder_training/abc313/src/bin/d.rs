// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

fn main() {
    let (N, K) = get!(usize, usize);

    // A1 ~ AK+1 までを復元
    let S = (0..=K)
        .map(|i| {
            // i, i + 1, ... (K個)
            println!("? {}", (i..i + K).map(|x| x % (K + 1) + 1).join(" "));
            // 結果の受け取り
            get!(usize)
        })
        .collect_vec();

    let SK1 = S.iter().fold(0, |a, b| a ^ b);

    let mut A = (0..=K).map(|i| SK1 ^ S[(i + 1) % (K + 1)]).collect_vec();

    debug!(A);

    // 0..K
    let SK = A[..K - 1].iter().sum::<usize>() % 2;
    debug!(SK);

    // 残りを復元
    for i in K + 1..N {
        // 1, 2, ..., K, i
        println!("? {} {}", (1..K).join(" "), i + 1);
        let res = get!(usize);
        A.push(res ^ SK);
    }

    println!("! {}", A.iter().join(" "));
}

const INF: usize = 1001001001001001001;
