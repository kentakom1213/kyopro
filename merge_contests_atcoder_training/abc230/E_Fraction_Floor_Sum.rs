//          E - Fraction Floor Sum         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc230/tasks/abc230_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;


// solve
fn main() {
    let N = get!(usize);
    
    // 約数列挙
    let mut divs = vec![];
    for i in 1.. {
        if i*i > N {break;}
        if N % i == 0 {
            if i*i == N {
                divs.push(i);
                break;
            } else {
                divs.push(i);
                divs.push(N/i);
            }
        }
    }
    divs.sort();

    let mut ans = N;
    for i in 0..divs.len()-1 {
        let tmp = N / divs[i+1] * (divs[i+1] - divs[i]);
        ans += tmp;
        eprintln!("{}/{} * ({}-{}) = {}", N, divs[i+1], divs[i+1], divs[i], tmp);
    }

        
    let k = {
        let mut k = 0;
        for i in 0.. {
            if i*i <= N {
                k = i;
            } else {
                break;
            }
        }
        k
    };

    for i in 1..=N/(k+1) {
        ans += N / i;
    }

    println!("{}", ans);
}

