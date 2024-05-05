//          C - The Kth Time Query         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc235/tasks/abc235_c
// ----------------------------------------

use std::collections::HashMap;

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
    let (n, q) = get!(usize, usize);
    let a = get!(usize;;);

    let mut map = HashMap::new();
    for (i, a) in a.iter().enumerate() {
        if !map.contains_key(a) {
            map.insert(a, vec![i+1]);
        } else if let Some(arr) = map.get_mut(a) {
            (*arr).push(i+1);
        }
    }

    for i in 0..q {
        let (x, mut k) = get!(usize, usize);
        k -= 1;
        if !map.contains_key(&x) {
            println!("-1");
        } else {
            if k >= map[&x].len() {
                println!("-1");
            } else {
                println!("{}", map[&x][k]);
            }
        }
    }
}
