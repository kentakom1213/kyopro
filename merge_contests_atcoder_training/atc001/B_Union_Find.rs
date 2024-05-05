//              B - Union Find
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/atc001/tasks/unionfind_a
// ----------------------------------------

use proconio::input;

pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, u: usize) -> usize {
        if self.par[u] == u {
            u
        } else {
            self.par[u] = self.root(self.par[u]); // 経路を圧縮
            self.par[u]
        }
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.root(u);
        let mut v = self.root(v);

        if u == v {
            false
        } else {
            if self.rank[u] < self.rank[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.rank[u] == self.rank[v] {
                self.rank[u] += 1;
            }
            // rankが低い方にマージ
            self.par[v] = u;
            true
        }
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // UnionFind配列
    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        input! {
            com: usize,
            x: usize,
            y: usize,
        }

        if com == 0 {
            uf.merge(x, y);
        } else {
            let ans = uf.is_same(x, y);
            println!("{}", ["No", "Yes"][ans as usize]);
        }
    }
}
