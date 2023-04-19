//              E - Last Rook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc269/tasks/abc269_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

// imports
use itertools::Itertools;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// 行と列に関してそれぞれ2分探索を行う
fn main() {
    let N = get!(usize);

    let (mut b, mut t) = (1_usize, N);
    let (mut l, mut r) = (1_usize, N);

    // 行に関して2分探索
    while b < t {
        let m = b + t >> 1;
        println!("? {} {} {} {}", b, m, 1, N);
        let tmp = get!(usize);
        // b~m が大丈夫かどうか
        if m - b + 1 > tmp {
            t = m;
        } else {
            b = m + 1;
        }
    }

    // 列に関して2分探索
    while l < r {
        let m = l + r >> 1;
        println!("? {} {} {} {}", 1, N, l, m);
        let tmp = get!(usize);
        if m - l + 1 > tmp {
            r = m;
        } else {
            l = m + 1;
        }
    }

    println!("! {} {}", b, l);
}