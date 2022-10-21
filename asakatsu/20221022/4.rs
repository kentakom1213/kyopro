// https://atcoder.jp/contests/abc035/tasks/abc035_b

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

#[derive(Debug)]
struct Dir {
    l: isize,
    r: isize,
    u: isize,
    d: isize,
    hatena: isize,
}

// solve
fn main() {
    let S = get!(String);
    let T = get!(usize);

    let mut dir = Dir{ l:0, r:0, u:0, d:0, hatena:0 };
    for c in S.chars() {
        match c {
            'L' => { dir.l += 1 },
            'R' => { dir.r += 1 },
            'U' => { dir.u += 1 },
            'D' => { dir.d += 1 },
            _ => { dir.hatena += 1 },
        }
    }

    // max
    if T == 1 {
        let x = dir.l.abs().max( dir.r.abs() );
        let y = dir.u.abs().max( dir.d.abs() );
        println!("{}", x + y + dir.hatena);
    } else {
        let x = (dir.l - dir.r).abs();
        let y = (dir.u - dir.d).abs();
        let rem = dir.hatena % 2;
        println!("{}", (x + y - 1).abs());
    }
}

