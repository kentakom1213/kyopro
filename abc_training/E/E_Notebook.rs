//               E - Notebook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc273/tasks/abc273_e
// ----------------------------------------

#![allow(non_snake_case)]

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


use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    val: isize,
    par: usize,
    vec: Vec<usize>,
}

fn main() {
    let Q = get!(usize);

    let mut tree: Vec<Node> = vec![ Node{val: -1, par: 0, vec: vec![] } ];
    let mut cur: usize = 0;
    let mut map = HashMap::new();

    for _ in 0..Q {
        let q = get!(String);

        match &q[0..1] {
            "A" => {
                let x = q.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap();

                tree.push( Node{val: x, par: cur, vec: vec![] } );

                let ins: usize = tree.len() - 1;
                tree[cur].vec.push(ins);
                cur = tree.len() - 1;
            },
            "D" => {
                cur = tree[cur].par;
            },
            "S" => {
                let x = q.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap();
                map.insert(x, cur);
            },
            "L" => {
                let x = q.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap();
                cur = match map.get(&x) {
                    Some(i) => *i,
                    None => 0,
                };
            },
            _ => unreachable!(),
        }
        print!("{} ", tree[cur].val);
    }
    println!();
}
