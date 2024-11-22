#![allow(non_snake_case)]

use cp_library_rs::{
    data_structure::union_find::UnionFind, debug, number_theory::modint::MOD998, utils::consts::INF,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        queries: [(usize, usize, usize); Q]
    }

    let mut X = 0;
    let mut uf = UnionFind::new(N);
    let mut parent = vec![INF; N];

    for (a, b, c) in queries {
        let t = (1 + X) * a % MOD998 % 2 + 1;
        let mut u = (1 + X) * b % MOD998 % N;
        let mut v = (1 + X) * c % MOD998 % N;
        debug!(t, u, v);

        if t == 1 {
            // マージテク
            if uf.get_size(u) < uf.get_size(v) {
                (u, v) = (v, u);
            }
            uf.unite(u, v);
            // uの方にマージ
            while parent[v] != INF {
                // 親子関係を反転
                let vp = parent[v];
                parent[v] = u;
                u = v;
                v = vp;
            }
            parent[v] = u;
        } else {
            X = 0;
            let pu = parent[u];
            let pv = parent[v];
            if pu != INF && pu == pv {
                X = pu + 1;
            }
            if pu != INF && parent[pu] == v {
                X = pu + 1;
            }
            if pv != INF && parent[pv] == u {
                X = pv + 1;
            }
            println!("{X}");
        }
    }
}
