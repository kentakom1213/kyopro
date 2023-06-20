//               Disjoint Set
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
// ----------------------------------------

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

/// ## UnionFind
pub mod disjoint_set {
    /// 2要素をマージする
    pub fn unite(par: &mut Vec<usize>, u: usize, v: usize) -> bool {
        let u = root(par, u);
        let v = root(par, v);

        if u == v {
            false
        } else {
            par[u] = v;
            true
        }
    }

    pub fn root(par: &mut Vec<usize>, u: usize) -> usize {
        if par[u] == u {
            u
        } else {
            par[u] = root(par, par[u]);
            par[u]
        }
    }

    /// 2要素が同一集合か判定
    pub fn is_same(par: &mut Vec<usize>, u: usize, v: usize) -> bool {
        root(par, u) == root(par, v)
    }
}

use disjoint_set::*;

fn main() {
    let (n, q) = get!(usize, usize);

    // UnionFind配列
    let mut uf: Vec<usize> = (0..n).collect();

    for _ in 0..q {
        let (com, x, y) = get!(usize, usize, usize);

        if com == 0 {
            unite(&mut uf, x, y);
        } else {
            let ans = is_same(&mut uf, x, y);
            println!("{}", ans as usize);
        }
    }
}
