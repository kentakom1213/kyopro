// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

pub type Graph = Vec<Vec<usize>>;

#[derive(Debug)]
pub struct Namori {
    pub N: usize,
    pub graph: Graph,
    pub forest: Graph,
    pub on_cycle: Vec<bool>,
}

impl Namori {
    pub fn new(N: usize) -> Self {
        Self {
            N,
            graph: vec![vec![]; N],
            forest: vec![vec![]; N],
            on_cycle: vec![true; N],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    pub fn decompose(&mut self) {
        // 葉を調べる
        let mut degree = vec![0; self.N];
        let mut leafs = VecDeque::new();
        let mut visited = vec![false; self.N];
        for i in 0..self.N {
            degree[i] = self.graph[i].len(); // 次数を調べる
            // 次数が1の頂点を格納
            if degree[i] == 1 {
                leafs.push_back(i);
                visited[i] = true;
            }
        }
        // 葉を辿って木に分解
        while let Some(u) = leafs.pop_front() {
            self.on_cycle[u] = false;
            for &v in &self.graph[u] {
                if visited[v] {
                    continue;
                }
                degree[v] -= 1;
                // 森に追加
                self.forest[u].push(v);
                self.forest[v].push(u);
                if degree[v] <= 1 {
                    leafs.push_back(v);
                    visited[v] = true;
                }
            }
        }
    }
}


// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// main
fn main() {
    input! {
        N: usize,
        A: [Usize1; N],
    }

    // グラフの構築
    let mut namori = Namori::new(N);
    for (u, &v) in A.iter().enumerate() {
        namori.add_edge(u, v);
    }

    // 分解
    namori.decompose();

    // 閉路上の頂点を取得
    let mut cur = 0;
    for i in 0..N {
        if namori.on_cycle[i] {
            cur = i;
            break;
        }
    }

    // 順に表示
    let mut visited = vec![false; N];
    visited[cur] = true;

    let mut ans = vec![cur];

    loop {
        let nxt = A[cur];
        if visited[nxt] {
            break;
        }
        ans.push(nxt);
        visited[nxt] = true;
        cur = nxt;
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().map(|&v| v + 1).join(" "));
}
