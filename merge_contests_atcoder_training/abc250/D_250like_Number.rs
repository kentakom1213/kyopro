//           D - 250-like Number           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc250/tasks/abc250_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

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


// solve
fn main() {
    let N = get!(usize);

    // 素数の列挙
    const MAX: usize = 1001001;
    let mut sieve = [1; MAX];
    sieve[0] = 0;
    sieve[1] = 0;
    for i in 2..MAX {
        for j in 2.. {
            if i*j >= MAX { break; }
            sieve[i*j] = 0;
        }
    }
    
    let primes: Vec<usize> = sieve.iter()
                                  .enumerate()
                                  .filter(|(i, &n)| n == 1)
                                  .map(|(i, &n)| i)
                                  .collect();

    // 全探索
    let mut ans = 0;
    for i in 0.. {
        let q = primes[i];
        if q.pow(3) >= N { break; }
        for j in 0..i {
            let p = primes[j];
            if p*q.pow(3) > N { break; }
            // println!("{}*{}^3 = {}", p, q, p*q.pow(3));
            ans += 1;
        }
    }

    println!("{}", ans);
}
