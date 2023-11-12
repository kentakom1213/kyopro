// https://mojacoder.app/users/shinnshinn/contests/ochacon02/tasks/1

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

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

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
pub fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}
/// ## chr
/// `chr(0) = A`であるようなascii文字(`A~Za~z`)を返す
pub fn chr(i: usize) -> char {
    let a = 'a' as u32;
    char::from_u32(a + i as u32).unwrap()
}

fn main() {
    let N = get!(usize);
    let C = get!(String).chars().map(ord).collect::<Vec<_>>();

    // 全区間での和
    let mut count_out = vec![0_usize; 26];
    for &c in &C[1..] {
        count_out[c] += 1;
    }

    let mut ans: usize = 0;

    // カウント
    for i in 0..N - 1 {
        let mut count_in = vec![0_usize; 26];
        for j in i + 1..N {
            // j文字目を区間から減らす
            count_out[C[j]] -= 1;
            if C[i] == C[j] {
                for k in 0..26 {
                    ans += count_in[k] * count_out[k];
                }
            }
            // j文字目を区間に追加
            count_in[C[j]] += 1;
        }
        count_in[C[i + 1]] -= 1;
        count_out = count_in;
    }

    println!("{}", ans);
}
