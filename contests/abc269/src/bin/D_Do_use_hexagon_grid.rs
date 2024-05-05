//         D - Do use hexagon grid         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc269/tasks/abc269_d
// ----------------------------------------

use proconio::input;
use std::collections::{ HashMap, HashSet };

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
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

    // 経路圧縮をしない`root`
    fn root_const(&self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.root_const(self.par[x])
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
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }

}

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    // 辞書に登録
    let mut map = HashMap::new();
    for (i, &tup) in xy.iter().enumerate() {
        map.insert(tup, i);
    }

    let mut uf = UnionFind::new(n);
    const adjacent: [(i32, i32); 6] = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];

    // それぞれの点について、隣接するマスが存在する場合結合させる
    for (i, (x, y)) in xy.iter().enumerate() {
        for &(dx, dy) in &adjacent {
            let nxt = (x+dx, y+dy);
            match map.get(&nxt) {
                Some(&j) => { uf.unite(i, j); },
                None => (),
            }
        }
    }

    let mut roots = HashSet::new();
    for child in &uf.par[..] {
        let parent = uf.root_const(*child);
        roots.insert(parent);
    }

    let ans = roots.len();
    println!("{}", ans);
}

