// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    group_count: usize,
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
            group_count: n,
        }
    }

    // 根を求める
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);  // 経路圧縮
        self.par[x]
    }

    // 同一の集合に所属するか判定
    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // 要素を結合
    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        // 要素数が大きい方を子にすることで、高さを均等に保つ
        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        self.group_count -= 1;
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

type Graph = Vec<Vec<usize>>;

// main
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    // グラフの構築
    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    // 次数が3以上の頂点を選択
    let mut deg_3 = vec![];
    for i in 0..N {
        if G[i].len() >= 3 {
            deg_3.push(i);
        }
    }

    // データ保存用
    let mut ans = vec![]; // 答えを保存
    let mut visited = vec![INF; N];

    // 次数が3以上の頂点から1回で辿れる頂点を塗る
    for (i, &start) in deg_3.iter().enumerate() {
        let lev = coloring(start, i, &G, &mut visited);
        ans.push(lev);
    }

    // 残ったグラフを構築
    let mut remG = vec![vec![]; N];
    for &(u, v) in &edges {
        if visited[u] == INF && visited[v] == INF {
            remG[u].push(v);
            remG[v].push(u);
        }
    }

    // 連結成分に分割
    let mut color = vec![INF; N];
    for i in 0..N {
        bfs(i, i, &remG, &mut color);
    }

    debug!(&remG);
    debug!(&color);

    // 連結成分の大きさをカウント
    let mut size = vec![0; N];
    for &c in &color {
        size[c] += 1;
    }

    debug!(&size);

    for &s in &size {
        if s >= 3 {
            for _ in 0..s / 3 {
                ans.push(2);
            }
        }
    }

    // 出力
    println!("{}", ans.iter().sorted().join(" "));
}

/// startから1回で辿れる頂点を塗る
/// 星のレベルを返す
fn coloring(start: usize, color: usize, G: &Graph, array: &mut Vec<usize>) -> usize {
    array[start] = color;
    for &v in &G[start] {
        array[v] = color;
    }
    G[start].len()
}

/// BFSしてパスの長さを求める
fn bfs(start: usize, color: usize, G: &Graph, array: &mut Vec<usize>) {
    if array[start] != INF {
        return;
    }
    array[start] = color;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        for &v in &G[u] {
            if array[v] == INF {
                array[v] = color;
                q.push_back(v);
            }
        }
    }
}

