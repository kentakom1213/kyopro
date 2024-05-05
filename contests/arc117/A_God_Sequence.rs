//             A - God Sequence            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc117/tasks/arc117_a
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

/*

## 方針
- 構築問題

*/

// solve
fn main() {
    let (A, B) = get!(isize, isize);

    let maxi = A.max(B);
    let mini = A.min(B);

    let mut vec1 = vec![];
    let mut sum = 0;
    for i in 1..=maxi {
        vec1.push(i);
        sum += i;
    }

    let mut vec2 = vec![];
    for i in 1..=mini-1 {
        vec2.push(i);
        sum -= i;
    }
    vec2.push(sum);

    // 表示
    let mut sign = if maxi == A {1} else {-1};
    for x in vec1 {
        print!("{} ", sign * x);
    }
    sign *= -1;
    for x in vec2 {
        print!("{} ", sign * x);
    }
    println!();
}

