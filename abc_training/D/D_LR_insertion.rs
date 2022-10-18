//             D - LR insertion            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc237/tasks/abc237_d
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


// LinkedList
#[derive(Clone, Debug)]
struct Node {
    val: usize,
    prev: usize,
    next: usize,
}

fn main() {
    let N = get!(usize);
    let S = get!(String);

    const INF: usize = 1_000_000_000;
    let mut list = vec![ Node{val: 0, prev: INF, next: INF} ; N+1 ];
    let mut root = 0;
    let mut cur = 0;

    for (i, c) in S.chars().enumerate() {
        let new = i + 1;
        list[new].val = i + 1;

        if c == 'R' {
            let nxt = list[cur].next;
            if nxt < INF {
                list[nxt].prev = new;
            }
            list[cur].next = new;
            list[new].prev = cur;
            list[new].next = nxt;
        } else {
            let prv = list[cur].prev;
            if prv < INF {
                list[prv].next = new;
            }
            list[cur].prev = new;
            list[new].prev = prv;
            list[new].next = cur;
            if root == cur {
                root = new;
            }
        }
        cur = new;
    }

    let mut idx = root;
    loop {
        print!("{} ", list[idx].val);
        if list[idx].next < INF {
            idx = list[idx].next;
        } else {
            break;
        }
    }
    println!();
}
