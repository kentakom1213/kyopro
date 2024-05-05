//             E - Through Path            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc187/tasks/abc187_e
// ----------------------------------------

/*

## 方針
- いもす法的な感じ

*/

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;


// solve
fn main() {
    let N = get!(usize);
    let mut edge = vec![];
    let mut G = vec![vec![]; N];
    for _ in 0..N-1 {
        let (a, b) = get!(usize, usize);
        edge.push((a-1, b-1));
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    
    // 根付き木にする（dfs）
    let mut depth = vec![-1; N];
    depth[0] = 0;
    let mut st = vec![0];
    while let Some(u) = st.pop() {
        for &v in &G[u] {
            if depth[v] == -1 {
                depth[v] = depth[u] + 1;
                st.push(v);
            }
        }
    }

    // いもす法
    let mut ans = vec![0_isize; N];
    let Q = get!(usize);
    for _ in 0..Q {
        let (mut t, e, x) = get!(usize, usize, isize);
        let (mut a, mut b) = edge[e-1];
        if depth[a] > depth[b] {
            std::mem::swap(&mut a, &mut b);
            t = t ^ 3;  // 1->2, 2->1
        }
        if t == 1 {
            ans[0] += x;
            ans[b] -= x;
        }
        if t == 2 {
            ans[b] += x;
        }
    }

    // 累積和をとる（dfs）
    let mut st = vec![0];
    while let Some(u) = st.pop() {
        for &v in &G[u] {
            // 根->葉の方向に足し合わせる
            if depth[u] < depth[v] {
                ans[v] += ans[u];
                st.push(v);
            }
        }
    }

    // println!("{:?}", depth);
    // println!("{:?}", ans);

    for &v in &ans {
        println!("{}", v);
    }
}
