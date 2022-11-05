// https://atcoder.jp/contests/abc087/tasks/arc090_a

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
static INF: usize = 1_000_000_000_000_000_000;


// solve
fn main() {
    let N = get!(usize);
    let A1 = get!(usize;;);
    let A2 = get!(usize;;);
    
    // 累積和をとる
    let S1 = {
        let mut s = vec![0; N+1];
        for i in 0..N {
            s[i+1] = s[i] + A1[i];
        }
        s
    };

    let S2 = {
        let mut s = vec![0; N+1];
        for i in 0..N {
            s[i+1] = s[i] + A2[i];
        }
        s
    };

    eprintln!("{:?}", S1);
    eprintln!("{:?}", S2);

    let mut ans = 0;
    for i in 0..=N {
        let tmp = (S1[i] - S1[0]) + ( S2[N] - S2[1.max(i)-1] );
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}

