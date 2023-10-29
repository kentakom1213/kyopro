//             D - LR insertion            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc237/tasks/abc237_d
// ----------------------------------------

#![allow(non_snake_case)]

use proconio::input;

// LinkedList
#[derive(Clone, Debug)]
struct Node {
    val: usize,
    prev: usize,
    next: usize,
}

fn main() {
    input! {
        N: usize,
        S: String,
    }

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
