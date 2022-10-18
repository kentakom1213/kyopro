// https://atcoder.jp/contests/abc194/tasks/abc194_e

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque};

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
    let (N, M) = get!(usize, usize);
    let A = get!(usize ;;);

    let mut pos = vec![vec![0]; N+1];
    for (i, &a_j) in A.iter().enumerate() {
        pos[a_j].push(i+1);
    }
    for i in 0..=N {
        pos[i].push(N+1);
    }

    // println!("{:?}", pos);

    // それぞれの値に条件を満たす区間が存在するか
    for i in 0..=N {
        let max = pos[i].len() - 1;
        for j in 0..max {
            if pos[i][j+1] - pos[i][j] > M {
                println!("{}", i);
                return;
            }
        }
    }
}

