// https://judge.yosupo.jp/problem/predecessor_problem

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    let (N, Q) = get!(usize, usize);
    let T = get!(String);

    let mut set = T
        .chars()
        .enumerate()
        .fold(BTreeSet::new(), |mut set, (x, e)| {
            if e == '1' {
                set.insert(x as isize);
            }
            set
        });

    for _ in 0..Q {
        let (c, k) = get!(usize, isize);

        match c {
            0 => {
                if set.get(&k).is_none() {
                    set.insert(k);
                }
            }
            1 => {
                set.remove(&k);
            }
            2 => {
                println!("{}", set.contains(&k) as usize);
            }
            3 => {
                let res = *set.range(k..).next().unwrap_or(&(-1));
                println!("{res}");
            }
            4 => {
                let res = *set.range(..=k).next_back().unwrap_or(&(-1));
                println!("{res}");
            }
            _ => (),
        }
    }
}
