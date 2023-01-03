#![allow(dead_code)]

// [RustでUnionFind](https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb)
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    cnt: usize,
}

impl UnionFind {
    // UnionFindを新規作成
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
            cnt: n,
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
        self.cnt -= 1;
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unionfind() {
        let mut uf = UnionFind::new(8);
        /*
         * 0 1 2 3 4 5 6 7
         */

        uf.unite(1, 2);
        uf.unite(3, 2);
        /*
         * 0 1-2-3 4 5 6 7
         */

        assert_eq!(uf.cnt, 6);

        assert!(uf.issame(1, 3) == true);
        assert!(uf.issame(1, 4) == false);
        assert_eq!(uf.size(1), 3);

        uf.unite(2, 4);
        /*
         * 0 1-2-3-4 5 6 7
         */

        assert_eq!(uf.cnt, 5);
        assert!(uf.issame(4, 1) == true);
        assert_eq!(uf.size(4), 4);

        uf.unite(4, 2);
        uf.unite(0, 0);
        /*
         * 0 1-2-3-4 5 6 7
         */

        assert_eq!(uf.cnt, 5);
        assert!(uf.issame(0, 0) == true);

        uf.unite(0, 7);
        /*
         * 0 1-2-3-4 5 6 7
         * └─────────────┘
         */

        assert_eq!(uf.cnt, 4);
        assert!(uf.issame(0, 7) == true);

        uf.unite(5, 6);
        /*
         * 0 1-2-3-4 5-6 7
         * └─────────────┘
         */

        assert_eq!(uf.cnt, 3);
        assert!(uf.issame(5, 6) == true);
        assert!(uf.issame(5, 7) == false);

        uf.unite(4, 5);
        uf.unite(6, 7);
        /*
         * 0-1-2-3-4-5-6-7
         */

        assert_eq!(uf.cnt, 1);

        uf.unite(0, 7);
        /*
         * 0-1-2-3-4-5-6-7
         */
    
        assert_eq!(uf.cnt, 1);
    }
}
